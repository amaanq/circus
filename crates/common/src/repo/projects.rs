use circus_codegen::queries::projects as q;
use uuid::Uuid;

use crate::{
  db::{PgPool, is_unique_violation},
  error::{CiError, Result},
  models::{CreateProject, Project, UpdateProject},
};

impl From<q::ProjectRow> for Project {
  fn from(r: q::ProjectRow) -> Self {
    Self {
      id:             r.id,
      name:           r.name,
      description:    r.description,
      repository_url: r.repository_url,
      created_at:     r.created_at,
      updated_at:     r.updated_at,
    }
  }
}

/// Create a new project.
///
/// # Errors
///
/// Returns error if database insert fails or project name already exists.
pub async fn create(pool: &PgPool, input: CreateProject) -> Result<Project> {
  let client = pool.get().await?;
  q::create()
    .bind(
      &client,
      &input.name,
      &input.description,
      &input.repository_url,
    )
    .one()
    .await
    .map(Project::from)
    .map_err(|e| {
      if is_unique_violation(&e) {
        CiError::Conflict(format!("Project '{}' already exists", input.name))
      } else {
        CiError::Database(e)
      }
    })
}

/// Get a project by ID.
///
/// # Errors
///
/// Returns error if database query fails or project not found.
pub async fn get(pool: &PgPool, id: Uuid) -> Result<Project> {
  let client = pool.get().await?;
  q::get()
    .bind(&client, &id)
    .opt()
    .await?
    .map(Project::from)
    .ok_or_else(|| CiError::NotFound(format!("Project {id} not found")))
}

/// Get a project by name.
///
/// # Errors
///
/// Returns error if database query fails or project not found.
pub async fn get_by_name(pool: &PgPool, name: &str) -> Result<Project> {
  let client = pool.get().await?;
  q::get_by_name()
    .bind(&client, &name)
    .opt()
    .await?
    .map(Project::from)
    .ok_or_else(|| CiError::NotFound(format!("Project '{name}' not found")))
}

/// List projects with pagination.
///
/// # Errors
///
/// Returns error if database query fails.
pub async fn list(
  pool: &PgPool,
  limit: i64,
  offset: i64,
) -> Result<Vec<Project>> {
  let client = pool.get().await?;
  let rows = q::list().bind(&client, &limit, &offset).all().await?;
  Ok(rows.into_iter().map(Project::from).collect())
}

/// Count total number of projects.
///
/// # Errors
///
/// Returns error if database query fails.
pub async fn count(pool: &PgPool) -> Result<i64> {
  let client = pool.get().await?;
  Ok(q::count().bind(&client).one().await?)
}

/// Update a project with partial fields.
///
/// # Errors
///
/// Returns error if database update fails or project not found.
pub async fn update(
  pool: &PgPool,
  id: Uuid,
  input: UpdateProject,
) -> Result<Project> {
  // Read-modify-write so omitted fields keep their existing value.
  let existing = get(pool, id).await?;
  let name = input.name.unwrap_or(existing.name);
  let description = input.description.or(existing.description);
  let repository_url = input.repository_url.unwrap_or(existing.repository_url);

  let client = pool.get().await?;
  q::update()
    .bind(&client, &name, &description, &repository_url, &id)
    .one()
    .await
    .map(Project::from)
    .map_err(|e| {
      if is_unique_violation(&e) {
        CiError::Conflict(format!("Project '{name}' already exists"))
      } else {
        CiError::Database(e)
      }
    })
}

/// Insert or update a project by name.
///
/// # Errors
///
/// Returns error if database operation fails.
pub async fn upsert(pool: &PgPool, input: CreateProject) -> Result<Project> {
  let client = pool.get().await?;
  Ok(
    q::upsert()
      .bind(
        &client,
        &input.name,
        &input.description,
        &input.repository_url,
      )
      .one()
      .await
      .map(Project::from)?,
  )
}

/// List projects that have no jobsets at all.
///
/// # Errors
///
/// Returns error if database query fails.
pub async fn list_without_active_jobsets(
  pool: &PgPool,
) -> Result<Vec<Project>> {
  let client = pool.get().await?;
  let rows = q::list_without_active_jobsets().bind(&client).all().await?;
  Ok(rows.into_iter().map(Project::from).collect())
}

/// Delete a project by ID.
///
/// # Errors
///
/// Returns error if database delete fails or project not found.
pub async fn delete(pool: &PgPool, id: Uuid) -> Result<()> {
  let client = pool.get().await?;
  let affected = q::delete().bind(&client, &id).await?;
  if affected == 0 {
    return Err(CiError::NotFound(format!("Project {id} not found")));
  }
  Ok(())
}
