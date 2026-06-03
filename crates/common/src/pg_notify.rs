// Subscribes to Postgres LISTEN/NOTIFY channels on a dedicated tokio-postgres
// connection and signals an `Arc<tokio::sync::Notify>` when events arrive.
// Daemons use this to wake immediately instead of waiting for the next poll
// interval.
//
// A pooled (deadpool) connection cannot be used here: the pool drives each
// connection's message stream internally and discards async notifications, so
// LISTEN payloads never surface. We therefore open and drive our own
// connection and read `AsyncMessage::Notification` off its message stream.
use std::{sync::Arc, time::Duration};

use futures::{StreamExt as _, stream};
use tokio::{sync::Notify, task::JoinHandle};
use tokio_postgres::{AsyncMessage, NoTls};

use crate::db::TlsMode;

/// Channel emitted on `builds` INSERT or status UPDATE.
pub const CHANNEL_BUILDS_CHANGED: &str = "circus_builds_changed";

/// Channel emitted on `jobsets` INSERT, UPDATE (relevant fields), or DELETE.
pub const CHANNEL_JOBSETS_CHANGED: &str = "circus_jobsets_changed";

/// Spawns a background task that listens on the given PG channels and signals
/// `wakeup` on each notification. Uses `notify_one` so a notification arriving
/// while the daemon is mid-cycle still wakes the next `.notified()` await.
/// Reconnects with 5s backoff on connection loss.
pub fn spawn_listener(
  database_url: &str,
  channels: &[&str],
  wakeup: Arc<Notify>,
) -> JoinHandle<()> {
  let database_url = database_url.to_owned();
  let channels: Vec<String> =
    channels.iter().map(|s| (*s).to_owned()).collect();

  tokio::spawn(async move {
    loop {
      if let Err(e) = listen_loop(&database_url, &channels, &wakeup).await {
        tracing::warn!("PG LISTEN connection lost: {e}, reconnecting in 5s");
      }
      tokio::time::sleep(Duration::from_secs(5)).await;
    }
  })
}

/// Core listen loop: connects, subscribes, and dispatches notifications. The
/// connection's message stream is driven on its own task; query progress and
/// notification delivery both flow through it.
async fn listen_loop(
  database_url: &str,
  channels: &[String],
  wakeup: &Notify,
) -> Result<(), tokio_postgres::Error> {
  let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<()>();

  let driver = match crate::db::tls_mode(database_url) {
    TlsMode::Disable => {
      let (client, conn) = tokio_postgres::connect(database_url, NoTls).await?;
      let driver = spawn_driver(conn, tx);
      subscribe(&client, channels).await?;
      driver
    },
    mode => {
      let connector = circus_migrations::tls::tls_connector(mode);
      let (client, conn) =
        tokio_postgres::connect(database_url, connector).await?;
      let driver = spawn_driver(conn, tx);
      subscribe(&client, channels).await?;
      driver
    },
  };

  tracing::info!(channels = ?channels, "PG LISTEN subscribed");

  // Each forwarded message is a notification; `notify_one` deposits a permit
  // so events arriving while the daemon is busy aren't lost. When the driver
  // task ends (connection dropped), the channel closes and we return to
  // reconnect.
  while rx.recv().await.is_some() {
    wakeup.notify_one();
  }

  driver.abort();
  Ok(())
}

/// Drive the connection's message stream on a background task, forwarding one
/// unit per notification. Returns the task handle.
fn spawn_driver<S>(
  mut connection: tokio_postgres::Connection<tokio_postgres::Socket, S>,
  tx: tokio::sync::mpsc::UnboundedSender<()>,
) -> JoinHandle<()>
where
  S: tokio_postgres::tls::TlsStream + Unpin + Send + 'static,
{
  tokio::spawn(async move {
    let mut messages = stream::poll_fn(move |cx| connection.poll_message(cx));
    while let Some(msg) = messages.next().await {
      match msg {
        Ok(AsyncMessage::Notification(_)) => {
          if tx.send(()).is_err() {
            break;
          }
        },
        Ok(_) => {},
        Err(_) => break,
      }
    }
  })
}

async fn subscribe(
  client: &tokio_postgres::Client,
  channels: &[String],
) -> Result<(), tokio_postgres::Error> {
  for channel in channels {
    // Channel names are fixed lowercase identifiers (see the assertions
    // below); quote to bind the exact name pg_notify() emits.
    let quoted = channel.replace('"', "\"\"");
    client
      .batch_execute(&format!("LISTEN \"{quoted}\""))
      .await?;
  }
  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn channel_names_are_valid_pg_identifiers() {
    for name in [CHANNEL_BUILDS_CHANGED, CHANNEL_JOBSETS_CHANGED] {
      assert!(name.len() < 64, "channel name too long: {name}");
      assert!(!name.contains(' '), "channel name has spaces: {name}");
      assert!(
        name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_'),
        "channel name has invalid chars: {name}"
      );
    }
  }

  #[test]
  fn channel_names_match_migration_triggers() {
    // These must match the pg_notify() calls in migration 015
    assert_eq!(CHANNEL_BUILDS_CHANGED, "circus_builds_changed");
    assert_eq!(CHANNEL_JOBSETS_CHANGED, "circus_jobsets_changed");
  }
}
