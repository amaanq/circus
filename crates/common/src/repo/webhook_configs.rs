use circus_codegen::queries::webhook_configs as q;
use uuid::Uuid;

use crate::{
  config::DeclarativeWebhook,
  db::{PgPool, is_unique_violation},
  error::{CiError, Result},
  models::{CreateWebhookConfig, WebhookConfig},
};

impl From<q::WebhookConfigRow> for WebhookConfig {
  fn from(r: q::WebhookConfigRow) -> Self {
    Self {
      id:          r.id,
      project_id:  r.project_id,
      forge_type:  r.forge_type,
      secret_hash: r.secret_hash,
      enabled:     r.enabled,
      created_at:  r.created_at,
    }
  }
}

/// Create a new webhook config.
///
/// `secret` is the raw webhook secret. Despite the underlying column
/// being called `secret_hash`, signature verification requires the
/// original secret (HMAC key for GitHub/Gitea/Forgejo, bearer token for
/// GitLab), so the value is stored as-is.
///
/// # Errors
///
/// Returns error if database insert fails or config already exists.
pub async fn create(
  pool: &PgPool,
  input: CreateWebhookConfig,
  secret: Option<&str>,
) -> Result<WebhookConfig> {
  let client = pool.get().await?;
  q::create()
    .bind(&client, &input.project_id, &input.forge_type, &secret)
    .one()
    .await
    .map(WebhookConfig::from)
    .map_err(|e| {
      if is_unique_violation(&e) {
        CiError::Conflict(format!(
          "Webhook config for forge '{}' already exists for this project",
          input.forge_type
        ))
      } else {
        CiError::Database(e)
      }
    })
}

/// Get a webhook config by ID.
///
/// # Errors
///
/// Returns error if database query fails or config not found.
pub async fn get(pool: &PgPool, id: Uuid) -> Result<WebhookConfig> {
  let client = pool.get().await?;
  q::get()
    .bind(&client, &id)
    .opt()
    .await?
    .map(WebhookConfig::from)
    .ok_or_else(|| CiError::NotFound(format!("Webhook config {id} not found")))
}

/// List all webhook configs for a project.
///
/// # Errors
///
/// Returns error if database query fails.
pub async fn list_for_project(
  pool: &PgPool,
  project_id: Uuid,
) -> Result<Vec<WebhookConfig>> {
  let client = pool.get().await?;
  let rows = q::list_for_project()
    .bind(&client, &project_id)
    .all()
    .await?;
  Ok(rows.into_iter().map(WebhookConfig::from).collect())
}

/// Get a webhook config by project and forge type.
///
/// # Errors
///
/// Returns error if database query fails.
pub async fn get_by_project_and_forge(
  pool: &PgPool,
  project_id: Uuid,
  forge_type: &str,
) -> Result<Option<WebhookConfig>> {
  let client = pool.get().await?;
  Ok(
    q::get_by_project_and_forge()
      .bind(&client, &project_id, &forge_type)
      .opt()
      .await?
      .map(WebhookConfig::from),
  )
}

/// Delete a webhook config.
///
/// # Errors
///
/// Returns error if database delete fails or config not found.
pub async fn delete(pool: &PgPool, id: Uuid) -> Result<()> {
  let client = pool.get().await?;
  let affected = q::delete().bind(&client, &id).await?;
  if affected == 0 {
    return Err(CiError::NotFound(format!("Webhook config {id} not found")));
  }
  Ok(())
}

/// Upsert a webhook config (insert or update on conflict).
///
/// `secret` is the raw webhook secret; see `create` for the rationale on
/// plaintext storage.
///
/// # Errors
///
/// Returns error if database operation fails.
pub async fn upsert(
  pool: &PgPool,
  project_id: Uuid,
  forge_type: &str,
  secret: Option<&str>,
  enabled: bool,
) -> Result<WebhookConfig> {
  let client = pool.get().await?;
  q::upsert()
    .bind(&client, &project_id, &forge_type, &secret, &enabled)
    .one()
    .await
    .map(WebhookConfig::from)
    .map_err(CiError::Database)
}

/// Sync webhook configs from declarative config.
/// Deletes configs not in the declarative list and upserts those that are.
///
/// # Errors
///
/// Returns error if database operations fail.
pub async fn sync_for_project(
  pool: &PgPool,
  project_id: Uuid,
  webhooks: &[DeclarativeWebhook],
  resolve_secret: impl Fn(&DeclarativeWebhook) -> Option<String>,
) -> Result<()> {
  // Get forge types from declarative config
  let types: Vec<&str> =
    webhooks.iter().map(|w| w.forge_type.as_str()).collect();

  // Delete webhook configs not in declarative config
  let client = pool.get().await?;
  q::sync_for_project_delete()
    .bind(&client, &project_id, &types)
    .await?;

  // Upsert each webhook config
  for webhook in webhooks {
    let secret = resolve_secret(webhook);

    upsert(
      pool,
      project_id,
      &webhook.forge_type,
      secret.as_deref(),
      webhook.enabled,
    )
    .await?;
  }

  Ok(())
}
