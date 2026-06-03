use chrono::{DateTime, Utc};
use circus_codegen::queries::build_products as q;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
  db::PgPool,
  error::{CiError, Result},
  models::{BuildProduct, BuildStatus, CreateBuildProduct},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PinnedBuildProduct {
  pub build_id:           Uuid,
  pub job_name:           String,
  pub system:             String,
  pub status:             BuildStatus,
  pub build_created_at:   DateTime<Utc>,
  pub product_id:         Uuid,
  pub product_name:       String,
  pub path:               String,
  pub gc_root_path:       Option<String>,
  pub product_created_at: DateTime<Utc>,
}

impl From<q::BuildProductRow> for BuildProduct {
  fn from(r: q::BuildProductRow) -> Self {
    Self {
      id:           r.id,
      build_id:     r.build_id,
      name:         r.name,
      path:         r.path,
      sha256_hash:  r.sha256_hash,
      file_size:    r.file_size,
      content_type: r.content_type,
      is_directory: r.is_directory,
      gc_root_path: r.gc_root_path,
      created_at:   r.created_at,
    }
  }
}

impl From<q::ListPinned> for PinnedBuildProduct {
  fn from(r: q::ListPinned) -> Self {
    Self {
      build_id:           r.build_id,
      job_name:           r.job_name,
      system:             r.system.unwrap_or_default(),
      status:             BuildStatus::from_db_str(
        r.status.as_deref().unwrap_or_default(),
      ),
      build_created_at:   r.build_created_at,
      product_id:         r.product_id,
      product_name:       r.product_name,
      path:               r.path,
      gc_root_path:       r.gc_root_path,
      product_created_at: r.product_created_at,
    }
  }
}

impl From<q::ListPinnedForGc> for PinnedBuildProduct {
  fn from(r: q::ListPinnedForGc) -> Self {
    Self {
      build_id:           r.build_id,
      job_name:           r.job_name,
      system:             r.system.unwrap_or_default(),
      status:             BuildStatus::from_db_str(
        r.status.as_deref().unwrap_or_default(),
      ),
      build_created_at:   r.build_created_at,
      product_id:         r.product_id,
      product_name:       r.product_name,
      path:               r.path,
      gc_root_path:       r.gc_root_path,
      product_created_at: r.product_created_at,
    }
  }
}

/// Create a build product record.
///
/// # Errors
///
/// Returns error if database insert fails.
pub async fn create(
  pool: &PgPool,
  input: CreateBuildProduct,
) -> Result<BuildProduct> {
  let client = pool.get().await?;
  Ok(
    q::create()
      .bind(
        &client,
        &input.build_id,
        &input.name,
        &input.path,
        &input.sha256_hash,
        &input.file_size,
        &input.content_type,
        &input.is_directory,
      )
      .one()
      .await
      .map(BuildProduct::from)?,
  )
}

/// Get a build product by ID.
///
/// # Errors
///
/// Returns error if database query fails or product not found.
pub async fn get(pool: &PgPool, id: Uuid) -> Result<BuildProduct> {
  let client = pool.get().await?;
  q::get()
    .bind(&client, &id)
    .opt()
    .await?
    .map(BuildProduct::from)
    .ok_or_else(|| CiError::NotFound(format!("Build product {id} not found")))
}

/// List all build products for a build.
///
/// # Errors
///
/// Returns error if database query fails.
pub async fn list_for_build(
  pool: &PgPool,
  build_id: Uuid,
) -> Result<Vec<BuildProduct>> {
  let client = pool.get().await?;
  let rows = q::list_for_build().bind(&client, &build_id).all().await?;
  Ok(rows.into_iter().map(BuildProduct::from).collect())
}

/// Set the GC-root path recorded for a product.
///
/// # Errors
///
/// Returns error if database update fails.
pub async fn set_gc_root_path(
  pool: &PgPool,
  id: Uuid,
  gc_root_path: Option<&str>,
) -> Result<()> {
  let client = pool.get().await?;
  q::set_gc_root_path()
    .bind(&client, &gc_root_path, &id)
    .await?;
  Ok(())
}

/// List products whose build has `keep = true`.
///
/// # Errors
///
/// Returns error if database query fails.
pub async fn list_pinned(
  pool: &PgPool,
  limit: i64,
  offset: i64,
) -> Result<Vec<PinnedBuildProduct>> {
  let client = pool.get().await?;
  let rows = q::list_pinned()
    .bind(&client, &limit, &offset)
    .all()
    .await?;
  Ok(rows.into_iter().map(PinnedBuildProduct::from).collect())
}

/// Count products whose build has `keep = true`.
///
/// # Errors
///
/// Returns error if database query fails.
pub async fn count_pinned(pool: &PgPool) -> Result<i64> {
  let client = pool.get().await?;
  Ok(q::count_pinned().bind(&client).one().await?)
}

/// List pinned products without pagination for GC preservation.
///
/// # Errors
///
/// Returns error if database query fails.
pub async fn list_pinned_for_gc(
  pool: &PgPool,
) -> Result<Vec<PinnedBuildProduct>> {
  let client = pool.get().await?;
  let rows = q::list_pinned_for_gc().bind(&client).all().await?;
  Ok(rows.into_iter().map(PinnedBuildProduct::from).collect())
}
