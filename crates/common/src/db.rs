//! Postgres connectivity: a deadpool-postgres pool and the generated query
//! client trait the repo layer runs against.
//!
//! TLS handling (driven by the URL's `sslmode`) lives in `circus-migrations`
//! and is reused here so the pool and the migration runner agree.

pub use circus_codegen::client::GenericClient;
pub use circus_migrations::tls::{TlsMode, connect_once, tls_mode};
use color_eyre::eyre::Context as _;
use tokio_postgres::NoTls;

/// The application connection pool. Repo functions acquire a client with
/// `pool.get().await?` and pass `&client` to the generated query builders.
pub type PgPool = deadpool_postgres::Pool;

/// A pooled connection checked out of [`PgPool`].
pub type DbClient = deadpool_postgres::Client;

/// Build a deadpool-postgres pool, honoring the URL's `sslmode`.
///
/// # Errors
///
/// Returns an error if the pool configuration is invalid.
pub fn build_pool(
  database_url: &str,
  max_size: usize,
) -> color_eyre::Result<PgPool> {
  use deadpool_postgres::{
    Config,
    ManagerConfig,
    PoolConfig,
    RecyclingMethod,
    Runtime,
  };

  let mut cfg = Config::new();
  cfg.url = Some(database_url.to_owned());
  cfg.manager = Some(ManagerConfig {
    recycling_method: RecyclingMethod::Fast,
  });
  cfg.pool = Some(PoolConfig {
    max_size,
    ..Default::default()
  });

  let pool = match tls_mode(database_url) {
    TlsMode::Disable => cfg.create_pool(Some(Runtime::Tokio1), NoTls),
    mode => {
      cfg.create_pool(
        Some(Runtime::Tokio1),
        circus_migrations::tls::tls_connector(mode),
      )
    },
  }
  .context("building postgres connection pool")?;

  Ok(pool)
}

/// Whether a tokio-postgres error is a unique-constraint violation (SQLSTATE
/// 23505). Repos use this to translate insert/update conflicts into
/// [`crate::error::CiError::Conflict`].
#[must_use]
pub fn is_unique_violation(err: &tokio_postgres::Error) -> bool {
  err.as_db_error().is_some_and(|db| {
    *db.code() == tokio_postgres::error::SqlState::UNIQUE_VIOLATION
  })
}
