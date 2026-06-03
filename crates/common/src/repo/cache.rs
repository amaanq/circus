use circus_codegen::queries::cache as q;

use crate::{db::PgPool, error::Result};

/// Look up a signed store path matching a `/nix/store/<hash>-...` LIKE pattern.
///
/// # Errors
///
/// Returns error if database query fails.
pub async fn find_signed_store_path(
  pool: &PgPool,
  like_pattern: &str,
) -> Result<Option<String>> {
  let client = pool.get().await?;

  if let Some(path) = q::find_signed_product_path()
    .bind(&client, &like_pattern)
    .opt()
    .await?
  {
    return Ok(Some(path));
  }

  Ok(
    q::find_signed_build_output_path()
      .bind(&client, &like_pattern)
      .opt()
      .await?,
  )
}
