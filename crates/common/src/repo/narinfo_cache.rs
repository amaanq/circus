//! Read/write of the `narinfo_cache` table.
//!
//! The runner's RPC server upserts a row here every time an agent
//! reports a successful presigned-NAR upload via
//! `Runner.notifyUploadComplete`. The server's cache route reads from
//! here when answering `<hash>.narinfo` queries, so a path uploaded by
//! any agent in the cluster is immediately visible to substituters.

use chrono::{DateTime, Utc};
use circus_codegen::queries::narinfo_cache as q;
use serde::{Deserialize, Serialize};

use crate::{
  db::PgPool,
  error::{CiError, Result},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NarInfo {
  pub store_path:  String,
  pub nar_hash:    String,
  pub nar_size:    i64,
  pub file_hash:   Option<String>,
  pub file_size:   Option<i64>,
  pub compression: String,
  pub url:         String,
  pub deriver:     Option<String>,
  pub references:  Vec<String>,
  pub sig:         Option<String>,
  pub ca:          Option<String>,
  pub created_at:  DateTime<Utc>,
  pub updated_at:  DateTime<Utc>,
}

impl From<q::NarinfoCacheRow> for NarInfo {
  fn from(r: q::NarinfoCacheRow) -> Self {
    Self {
      store_path:  r.store_path,
      nar_hash:    r.nar_hash,
      nar_size:    r.nar_size,
      file_hash:   r.file_hash,
      file_size:   r.file_size,
      compression: r.compression,
      url:         r.url,
      deriver:     r.deriver,
      references:  r.references,
      sig:         r.sig,
      ca:          r.ca,
      created_at:  r.created_at,
      updated_at:  r.updated_at,
    }
  }
}

pub struct UpsertNarInfo<'a> {
  pub store_path:  &'a str,
  pub nar_hash:    &'a str,
  pub nar_size:    i64,
  pub file_hash:   Option<&'a str>,
  pub file_size:   Option<i64>,
  pub compression: &'a str,
  pub url:         &'a str,
  pub deriver:     Option<&'a str>,
  pub references:  &'a [String],
  pub sig:         Option<&'a str>,
  pub ca:          Option<&'a str>,
}

/// Insert or replace the narinfo for one store path.
///
/// # Errors
/// Returns the underlying database error.
pub async fn upsert(pool: &PgPool, info: UpsertNarInfo<'_>) -> Result<()> {
  let client = pool.get().await?;
  q::upsert()
    .bind(
      &client,
      &info.store_path,
      &info.nar_hash,
      &info.nar_size,
      &info.file_hash,
      &info.file_size,
      &info.compression,
      &info.url,
      &info.deriver,
      &info.references,
      &info.sig,
      &info.ca,
    )
    .await?;
  Ok(())
}

/// Read the narinfo for one store path.
///
/// # Errors
/// `CiError::NotFound` when no row matches, `CiError::Database` for
/// underlying database errors.
pub async fn get(pool: &PgPool, store_path: &str) -> Result<NarInfo> {
  let client = pool.get().await?;
  q::get()
    .bind(&client, &store_path)
    .opt()
    .await?
    .map(NarInfo::from)
    .ok_or_else(|| CiError::NotFound(format!("narinfo for {store_path}")))
}

/// Lookup by the first 32 base32 characters of the store path's hash.
/// Substituters query `<hash>.narinfo`; this resolves that to a row.
///
/// # Errors
/// Same as [`get`].
pub async fn get_by_hash_part(
  pool: &PgPool,
  hash_part: &str,
) -> Result<NarInfo> {
  // Nix store paths are `/nix/store/<32-chars>-<name>`; we match on the
  // 32-char hash part right after the prefix.
  let client = pool.get().await?;
  let pattern = format!("/nix/store/{hash_part}-%");
  q::get_by_hash_part()
    .bind(&client, &pattern)
    .opt()
    .await?
    .map(NarInfo::from)
    .ok_or_else(|| CiError::NotFound(format!("narinfo for hash {hash_part}")))
}

/// Lookup by the narinfo `URL` field, e.g. `nar/<hash>.nar.zst`.
///
/// This is used by the server's `/nix-cache/nar/...` route to resolve NARs
/// uploaded by agents through the presigned S3 flow.
///
/// # Errors
/// Same as [`get`].
pub async fn get_by_url(pool: &PgPool, url: &str) -> Result<NarInfo> {
  let client = pool.get().await?;
  q::get_by_url()
    .bind(&client, &url)
    .opt()
    .await?
    .map(NarInfo::from)
    .ok_or_else(|| CiError::NotFound(format!("narinfo for URL {url}")))
}

/// Total rows. Cheap for admin and metrics surfaces.
///
/// # Errors
/// Returns the underlying database error.
pub async fn count(pool: &PgPool) -> Result<i64> {
  let client = pool.get().await?;
  Ok(q::count().bind(&client).one().await?)
}
