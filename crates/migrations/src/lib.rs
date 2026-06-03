//! Database migration utilities.
//!
//! Replaces the former `sqlx::migrate!` runner with a self-contained one over
//! tokio-postgres. The on-disk format is unchanged: migrations are the
//! numbered `migrations/NNNN_*.sql` files, and applied state is tracked in the
//! same `_sqlx_migrations` table sqlx used, with the identical Sha384 checksum
//! over the file contents — so databases previously migrated by sqlx continue
//! seamlessly.

use std::time::Instant;

use color_eyre::eyre::{Context, eyre};
use sha2::{Digest, Sha384};
use tokio_postgres::types::ToSql;
use tracing::{info, warn};

pub mod tls;

/// Advisory-lock key serializing concurrent migration runs (server +
/// queue-runner + evaluator all call `run_migrations` on startup).
const MIGRATION_LOCK_KEY: i64 = 7_764_786_970_022;

struct Migration {
  version:     i64,
  description: &'static str,
  sql:         &'static str,
}

macro_rules! migrations {
  ($(($version:expr, $name:literal)),+ $(,)?) => {
    &[$(Migration {
      version: $version,
      description: $name,
      sql: include_str!(concat!("../migrations/", $name, ".sql")),
    }),+]
  };
}

const MIGRATIONS: &[Migration] = migrations![
  (1, "0001_tables"),
  (2, "0002_indexes"),
  (3, "0003_triggers"),
  (4, "0004_views"),
  (5, "0005_builder_cpu_cores"),
  (6, "0006_active_jobsets_enabled_filter"),
  (7, "0007_news_and_fod"),
  (8, "0008_service_heartbeats"),
  (9, "0009_audit_log"),
  (10, "0010_evaluations_notify"),
  (11, "0011_build_meta"),
  (12, "0012_builder_sessions"),
  (13, "0013_build_required_features"),
  (14, "0014_narinfo_cache"),
  (15, "0015_jobset_trigger_modes"),
  (16, "0016_evaluation_visibility"),
  (17, "0017_narinfo_cache_url_index"),
  (18, "0018_build_stats_include_pending"),
];

/// Runs database migrations, creating the database first if it does not exist.
///
/// # Errors
///
/// Returns an error if the database cannot be created/connected, or if a
/// migration fails or diverges from a previously applied checksum.
pub async fn run_migrations(database_url: &str) -> color_eyre::Result<()> {
  info!("Starting database migrations");
  ensure_database_exists(database_url).await?;

  let mut client = tls::connect_once(database_url)
    .await
    .context("connecting to database for migrations")?;

  client
    .batch_execute(
      "CREATE TABLE IF NOT EXISTS _sqlx_migrations (
         version BIGINT PRIMARY KEY,
         description TEXT NOT NULL,
         installed_on TIMESTAMPTZ NOT NULL DEFAULT now(),
         success BOOLEAN NOT NULL,
         checksum BYTEA NOT NULL,
         execution_time BIGINT NOT NULL
       )",
    )
    .await
    .context("creating _sqlx_migrations table")?;

  client
    .execute("SELECT pg_advisory_lock($1)", &[&MIGRATION_LOCK_KEY])
    .await
    .context("acquiring migration advisory lock")?;

  let result = apply_pending(&mut client).await;

  let unlock = client
    .execute("SELECT pg_advisory_unlock($1)", &[&MIGRATION_LOCK_KEY])
    .await;

  result?;
  unlock.context("releasing migration advisory lock")?;
  info!("Database migrations completed successfully");
  Ok(())
}

/// Connect to the maintenance database and `CREATE DATABASE` the target if it
/// is missing. No-op when the database already exists.
async fn ensure_database_exists(database_url: &str) -> color_eyre::Result<()> {
  let mut url =
    url::Url::parse(database_url).context("parsing database URL")?;
  let dbname = url.path().trim_start_matches('/').to_owned();
  if dbname.is_empty() {
    return Err(eyre!("database URL has no database name"));
  }

  // Connect to the `postgres` maintenance database with the same credentials.
  url.set_path("/postgres");
  let admin_url = url.to_string();
  let client = tls::connect_once(&admin_url)
    .await
    .context("connecting to maintenance database")?;

  let exists = client
    .query_opt("SELECT 1 FROM pg_database WHERE datname = $1", &[&dbname])
    .await
    .context("checking whether database exists")?
    .is_some();

  if !exists {
    warn!(database = %dbname, "database does not exist, creating it");
    // Identifiers cannot be parameterized; quote to neutralize the name.
    let quoted = dbname.replace('"', "\"\"");
    client
      .batch_execute(&format!("CREATE DATABASE \"{quoted}\""))
      .await
      .context("creating database")?;
    info!(database = %dbname, "database created");
  }
  Ok(())
}

async fn apply_pending(
  client: &mut tokio_postgres::Client,
) -> color_eyre::Result<()> {
  for migration in MIGRATIONS {
    let checksum = Sha384::digest(migration.sql.as_bytes()).to_vec();
    let existing = client
      .query_opt(
        "SELECT checksum, success FROM _sqlx_migrations WHERE version = $1",
        &[&migration.version],
      )
      .await
      .context("reading migration state")?;

    if let Some(row) = existing {
      let stored: Vec<u8> = row.get("checksum");
      let success: bool = row.get("success");
      if !success {
        return Err(eyre!(
          "migration {} previously failed; manual intervention required",
          migration.version
        ));
      }
      if stored != checksum {
        return Err(eyre!(
          "migration {} checksum mismatch: the file changed after being \
           applied",
          migration.version
        ));
      }
      continue;
    }

    apply_one(client, migration, &checksum).await?;
  }
  Ok(())
}

async fn apply_one(
  client: &mut tokio_postgres::Client,
  migration: &Migration,
  checksum: &[u8],
) -> color_eyre::Result<()> {
  let started = Instant::now();
  let tx = client
    .transaction()
    .await
    .context("starting migration transaction")?;
  tx.batch_execute(migration.sql)
    .await
    .with_context(|| format!("applying migration {}", migration.version))?;
  let execution_time = i64::try_from(started.elapsed().as_nanos()).unwrap_or(0);
  let params: [&(dyn ToSql + Sync); 4] = [
    &migration.version,
    &migration.description,
    &checksum,
    &execution_time,
  ];
  tx.execute(
    "INSERT INTO _sqlx_migrations
       (version, description, success, checksum, execution_time)
     VALUES ($1, $2, TRUE, $3, $4)",
    &params,
  )
  .await
  .context("recording applied migration")?;
  tx.commit().await.context("committing migration")?;

  info!(
    version = migration.version,
    description = migration.description,
    "applied migration"
  );
  Ok(())
}

/// Validates that all required tables and views exist.
///
/// # Errors
///
/// Returns an error if a query fails or a required table/view is missing.
pub async fn validate_schema(
  client: &tokio_postgres::Client,
) -> color_eyre::Result<()> {
  info!("Validating database schema");

  for table in REQUIRED_TABLES {
    let row = client
      .query_one(
        "SELECT COUNT(*) FROM information_schema.tables
         WHERE table_name = $1 AND table_schema = 'public'",
        &[table],
      )
      .await?;
    let count: i64 = row.get(0);
    if count == 0 {
      return Err(eyre!("Required table '{table}' does not exist"));
    }
  }

  for view in REQUIRED_VIEWS {
    let row = client
      .query_one(
        "SELECT COUNT(*) FROM information_schema.views
         WHERE table_name = $1 AND table_schema = 'public'",
        &[view],
      )
      .await?;
    let count: i64 = row.get(0);
    if count == 0 {
      return Err(eyre!("Required view '{view}' does not exist"));
    }
  }

  info!("Database schema validation passed");
  Ok(())
}

/// Tables every migrated database must contain. Kept in sync with the SQL in
/// `migrations/`.
pub const REQUIRED_TABLES: &[&str] = &[
  "api_keys",
  "audit_log",
  "build_dependencies",
  "build_metrics",
  "build_outputs",
  "build_products",
  "build_steps",
  "builds",
  "channels",
  "evaluations",
  "failed_paths_cache",
  "jobset_inputs",
  "jobsets",
  "news",
  "notification_configs",
  "notification_tasks",
  "project_members",
  "projects",
  "remote_builders",
  "service_heartbeats",
  "starred_jobs",
  "user_sessions",
  "users",
  "webhook_configs",
];

/// Views every migrated database must contain.
pub const REQUIRED_VIEWS: &[&str] =
  &["active_jobsets", "build_metrics_summary", "build_stats"];

/// Static migration descriptors exposed for inspection by tests and tooling.
/// Returns `(version, name)` pairs in the order they are applied.
#[must_use]
pub fn migration_set() -> Vec<(i64, String)> {
  MIGRATIONS
    .iter()
    .map(|m| (m.version, m.description.to_string()))
    .collect()
}
