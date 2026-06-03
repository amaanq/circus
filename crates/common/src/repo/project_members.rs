//! Project members repository - for per-project permissions

use circus_codegen::queries::project_members as q;
use uuid::Uuid;

use crate::{
  config::DeclarativeProjectMember,
  db::{PgPool, is_unique_violation},
  error::{CiError, Result},
  models::{CreateProjectMember, ProjectMember, UpdateProjectMember},
  roles::VALID_PROJECT_ROLES,
  validation::validate_role,
};

impl From<q::ProjectMemberRow> for ProjectMember {
  fn from(r: q::ProjectMemberRow) -> Self {
    Self {
      id:         r.id,
      project_id: r.project_id,
      user_id:    r.user_id,
      role:       r.role,
      created_at: r.created_at,
    }
  }
}

/// Add a member to a project with role validation
///
/// # Errors
///
/// Returns error if validation fails or database insert fails.
pub async fn create(
  pool: &PgPool,
  project_id: Uuid,
  data: &CreateProjectMember,
) -> Result<ProjectMember> {
  // Validate role
  validate_role(&data.role, VALID_PROJECT_ROLES)
    .map_err(|e| CiError::Validation(e.to_string()))?;

  let client = pool.get().await?;
  q::create()
    .bind(&client, &project_id, &data.user_id, &data.role)
    .one()
    .await
    .map(ProjectMember::from)
    .map_err(|e| {
      if is_unique_violation(&e) {
        CiError::Conflict(
          "User is already a member of this project".to_string(),
        )
      } else {
        CiError::Database(e)
      }
    })
}

/// Get a project member by ID
///
/// # Errors
///
/// Returns error if database query fails or member not found.
pub async fn get(pool: &PgPool, id: Uuid) -> Result<ProjectMember> {
  let client = pool.get().await?;
  q::get()
    .bind(&client, &id)
    .opt()
    .await?
    .map(ProjectMember::from)
    .ok_or_else(|| CiError::NotFound(format!("Project member {id} not found")))
}

/// Get a project member by project and user
///
/// # Errors
///
/// Returns error if database query fails.
pub async fn get_by_project_and_user(
  pool: &PgPool,
  project_id: Uuid,
  user_id: Uuid,
) -> Result<Option<ProjectMember>> {
  let client = pool.get().await?;
  Ok(
    q::get_by_project_and_user()
      .bind(&client, &project_id, &user_id)
      .opt()
      .await?
      .map(ProjectMember::from),
  )
}

/// List all members of a project
///
/// # Errors
///
/// Returns error if database query fails.
pub async fn list_for_project(
  pool: &PgPool,
  project_id: Uuid,
) -> Result<Vec<ProjectMember>> {
  let client = pool.get().await?;
  let rows = q::list_for_project()
    .bind(&client, &project_id)
    .all()
    .await?;
  Ok(rows.into_iter().map(ProjectMember::from).collect())
}

/// List all projects a user is a member of
///
/// # Errors
///
/// Returns error if database query fails.
pub async fn list_for_user(
  pool: &PgPool,
  user_id: Uuid,
) -> Result<Vec<ProjectMember>> {
  let client = pool.get().await?;
  let rows = q::list_for_user().bind(&client, &user_id).all().await?;
  Ok(rows.into_iter().map(ProjectMember::from).collect())
}

/// Update a project member's role with validation
///
/// # Errors
///
/// Returns error if validation fails or database update fails.
pub async fn update(
  pool: &PgPool,
  id: Uuid,
  data: &UpdateProjectMember,
) -> Result<ProjectMember> {
  if let Some(ref role) = data.role {
    validate_role(role, VALID_PROJECT_ROLES)
      .map_err(|e| CiError::Validation(e.to_string()))?;

    let client = pool.get().await?;
    q::update()
      .bind(&client, role, &id)
      .opt()
      .await?
      .map(ProjectMember::from)
      .ok_or_else(|| {
        CiError::NotFound(format!("Project member {id} not found"))
      })
  } else {
    get(pool, id).await
  }
}

/// Remove a member from a project
///
/// # Errors
///
/// Returns error if database delete fails or member not found.
pub async fn delete(pool: &PgPool, id: Uuid) -> Result<()> {
  let client = pool.get().await?;
  let affected = q::delete().bind(&client, &id).await?;
  if affected == 0 {
    return Err(CiError::NotFound(format!("Project member {id} not found")));
  }
  Ok(())
}

/// Remove a specific user from a project
///
/// # Errors
///
/// Returns error if database delete fails or user not found.
pub async fn delete_by_project_and_user(
  pool: &PgPool,
  project_id: Uuid,
  user_id: Uuid,
) -> Result<()> {
  let client = pool.get().await?;
  let affected = q::delete_by_project_and_user()
    .bind(&client, &project_id, &user_id)
    .await?;
  if affected == 0 {
    return Err(CiError::NotFound(
      "User is not a member of this project".to_string(),
    ));
  }
  Ok(())
}

/// Check if a user has a specific role or higher in a project
///
/// # Errors
///
/// Returns error if database query fails.
pub async fn check_permission(
  pool: &PgPool,
  project_id: Uuid,
  user_id: Uuid,
  required_role: &str,
) -> Result<bool> {
  use crate::roles::has_project_permission;

  let member = get_by_project_and_user(pool, project_id, user_id).await?;

  if let Some(m) = member {
    Ok(has_project_permission(&m.role, required_role))
  } else {
    Ok(false)
  }
}

/// Upsert a project member (insert or update on conflict).
///
/// # Errors
///
/// Returns error if validation fails or database operation fails.
pub async fn upsert(
  pool: &PgPool,
  project_id: Uuid,
  user_id: Uuid,
  role: &str,
) -> Result<ProjectMember> {
  // Validate role
  validate_role(role, VALID_PROJECT_ROLES)
    .map_err(|e| CiError::Validation(e.to_string()))?;

  let client = pool.get().await?;
  Ok(
    q::upsert()
      .bind(&client, &project_id, &user_id, &role)
      .one()
      .await
      .map(ProjectMember::from)?,
  )
}

/// Sync project members from declarative config.
/// Deletes members not in the declarative list and upserts those that are.
///
/// # Errors
///
/// Returns error if database operations fail.
pub async fn sync_for_project(
  pool: &PgPool,
  project_id: Uuid,
  members: &[DeclarativeProjectMember],
  resolve_user: impl Fn(&str) -> Option<Uuid>,
) -> Result<()> {
  // Get user IDs from declarative config
  let user_ids: Vec<Uuid> = members
    .iter()
    .filter_map(|m| resolve_user(&m.username))
    .collect();

  // Delete members not in declarative config
  let client = pool.get().await?;
  q::sync_delete_removed()
    .bind(&client, &project_id, &user_ids)
    .await?;

  // Upsert each member
  for member in members {
    if let Some(user_id) = resolve_user(&member.username) {
      upsert(pool, project_id, user_id, &member.role).await?;
    } else {
      tracing::warn!(
          project_id = %project_id,
          username = %member.username,
          "Could not resolve user for declarative project member"
      );
    }
  }

  Ok(())
}
