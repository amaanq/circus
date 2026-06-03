//! Integration tests for the migrations crate.
//!
//! Tests that require a live `PostgreSQL` instance skip themselves cleanly when
//! no database is reachable, matching the pattern used in
//! `crates/common/tests/database_tests.rs`. The pure tests at the bottom of
//! this file run unconditionally.
//!
//! Set `CIRCUS_TEST_DATABASE_URL` to point tests at a specific database. The
//! default, `postgresql://postgres:password@localhost/circus_migrations_test`,
//! is suitable for the `nix develop` postgres dev shell.
#![expect(clippy::expect_used, clippy::print_stderr, reason = "Fine in tests")]

use circus_migrations::{
  REQUIRED_TABLES,
  REQUIRED_VIEWS,
  migration_set,
  run_migrations,
  tls::connect_once,
  validate_schema,
};

fn test_database_url() -> String {
  std::env::var("CIRCUS_TEST_DATABASE_URL").unwrap_or_else(|_| {
    "postgresql://postgres:password@localhost/circus_migrations_test"
      .to_string()
  })
}

/// Split a database URL into the maintenance (`/postgres`) URL and the target
/// database name, so we can run admin statements (`DROP`/`CREATE`/existence
/// checks) without first connecting to the database under test.
fn maintenance_url_and_dbname(url: &str) -> (String, String) {
  let mut parsed = url::Url::parse(url).expect("parse database URL");
  let dbname = parsed.path().trim_start_matches('/').to_owned();
  parsed.set_path("/postgres");
  (parsed.to_string(), dbname)
}

/// Whether the target database exists, queried against the maintenance DB.
async fn database_exists(url: &str) -> Result<bool, tokio_postgres::Error> {
  let (admin_url, dbname) = maintenance_url_and_dbname(url);
  let client = connect_once(&admin_url).await?;
  let exists = client
    .query_opt("SELECT 1 FROM pg_database WHERE datname = $1", &[&dbname])
    .await?
    .is_some();
  Ok(exists)
}

/// Drop the test database if it exists so each test run starts from a clean
/// state. Failure to drop is non-fatal -- if the connection cannot be made
/// the caller will detect that next and skip.
async fn reset_database(url: &str) {
  let (admin_url, dbname) = maintenance_url_and_dbname(url);
  let Ok(client) = connect_once(&admin_url).await else {
    return;
  };
  // Identifiers cannot be parameterized; quote to neutralize the name. FORCE
  // terminates any lingering connections so the drop cannot wedge.
  let quoted = dbname.replace('"', "\"\"");
  let _ = client
    .batch_execute(&format!(
      "DROP DATABASE IF EXISTS \"{quoted}\" WITH (FORCE)"
    ))
    .await;
}

/// Try to verify the server is reachable. Returns `None` (skip) if not.
async fn require_postgres(url: &str) -> Option<()> {
  // We can't connect to a non-existent DB, so check whether the server answers
  // at all by querying the maintenance database for the target's existence.
  match database_exists(url).await {
    Ok(_) => Some(()),
    Err(e) => {
      eprintln!("Skipping: no PostgreSQL reachable at {url}: {e}");
      None
    },
  }
}

#[tokio::test]
async fn migrations_create_required_tables_and_views() {
  let url = test_database_url();
  let Some(()) = require_postgres(&url).await else {
    return;
  };

  reset_database(&url).await;

  run_migrations(&url).await.expect("run_migrations");

  let client = connect_once(&url).await.expect("connect after migrate");

  validate_schema(&client).await.expect("validate_schema");

  // Belt and braces: explicitly check every table/view we claim to require.
  for table in REQUIRED_TABLES {
    let row = client
      .query_one(
        "SELECT COUNT(*) FROM information_schema.tables WHERE table_name = $1 \
         AND table_schema = 'public'",
        &[table],
      )
      .await
      .expect("count tables");
    let n: i64 = row.get(0);
    assert_eq!(n, 1, "missing required table {table}");
  }
  for view in REQUIRED_VIEWS {
    let row = client
      .query_one(
        "SELECT COUNT(*) FROM information_schema.views WHERE table_name = $1 \
         AND table_schema = 'public'",
        &[view],
      )
      .await
      .expect("count views");
    let n: i64 = row.get(0);
    assert_eq!(n, 1, "missing required view {view}");
  }
}

#[tokio::test]
async fn migrations_are_idempotent_when_run_twice() {
  let url = test_database_url();
  let Some(()) = require_postgres(&url).await else {
    return;
  };

  reset_database(&url).await;

  run_migrations(&url).await.expect("first run");
  // The hot path: a no-op replay must succeed against the same schema.
  run_migrations(&url).await.expect("second run");

  let client = connect_once(&url).await.expect("connect");
  validate_schema(&client)
    .await
    .expect("validate after replay");

  // Applied migrations are recorded in _sqlx_migrations; row count must equal
  // the static migration set length.
  let applied: i64 = client
    .query_one("SELECT COUNT(*) FROM _sqlx_migrations", &[])
    .await
    .expect("count applied")
    .get(0);
  assert_eq!(
    applied as usize,
    migration_set().len(),
    "applied count does not match static migration set"
  );
}

#[tokio::test]
async fn run_migrations_creates_database_if_missing() {
  let url = test_database_url();
  let Some(()) = require_postgres(&url).await else {
    return;
  };

  reset_database(&url).await;
  assert!(
    !database_exists(&url).await.expect("exists check"),
    "precondition: db should not exist"
  );

  run_migrations(&url).await.expect("run on missing db");

  assert!(
    database_exists(&url).await.expect("exists check"),
    "run_migrations did not create the database"
  );
}

/// Pure tests below: no postgres required, run on every host.
#[test]
fn migration_set_is_non_empty_and_strictly_increasing() {
  let set = migration_set();
  assert!(!set.is_empty(), "no migrations registered at compile time");

  let mut prev = i64::MIN;
  for (version, name) in &set {
    assert!(
      *version > prev,
      "migration versions must be strictly increasing; saw {version} ({name}) \
       after {prev}"
    );
    assert!(!name.is_empty(), "migration {version} has empty name");
    prev = *version;
  }
}

#[test]
fn required_tables_constant_has_no_duplicates_and_is_sorted() {
  let mut copy: Vec<&&str> = REQUIRED_TABLES.iter().collect();
  copy.sort();
  copy.dedup();
  assert_eq!(
    copy.len(),
    REQUIRED_TABLES.len(),
    "REQUIRED_TABLES contains duplicates"
  );

  let mut sorted: Vec<&&str> = REQUIRED_TABLES.iter().collect();
  sorted.sort();
  let original: Vec<&&str> = REQUIRED_TABLES.iter().collect();
  assert_eq!(sorted, original, "REQUIRED_TABLES should be sorted");
}

#[test]
fn required_views_constant_has_no_duplicates() {
  let mut copy: Vec<&&str> = REQUIRED_VIEWS.iter().collect();
  copy.sort();
  copy.dedup();
  assert_eq!(
    copy.len(),
    REQUIRED_VIEWS.len(),
    "REQUIRED_VIEWS contains duplicates"
  );
}
