//! Advanced search functionality for circus.
//!
//! These queries are built dynamically from optional filters, so they cannot be
//! expressed as static clorinde queries; they run directly against
//! tokio-postgres with a positional parameter vector. Rows are mapped to the
//! shared `models` types, converting the text-backed enum columns by hand.

use std::fmt::Write as _;

use circus_codegen::queries::search as search_q;
use tokio_postgres::{Row, types::ToSql};
use uuid::Uuid;

use crate::{
  db::PgPool,
  error::Result,
  models::{
    Build,
    BuildStatus,
    Evaluation,
    EvaluationStatus,
    EvaluationTriggerKind,
    Jobset,
    JobsetState,
    JobsetTriggerMode,
    Project,
  },
};

/// Search entity types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SearchEntity {
  Projects,
  Jobsets,
  Evaluations,
  Builds,
}

/// Sort order for search results
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortOrder {
  Asc,
  Desc,
}

/// Sort field for builds
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuildSortField {
  CreatedAt,
  JobName,
  Status,
  Priority,
}

/// Sort field for projects
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProjectSortField {
  Name,
  CreatedAt,
  LastEvaluation,
}

/// Build status filter
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuildStatusFilter {
  Pending,
  Running,
  Succeeded,
  Failed,
  Cancelled,
  DependencyFailed,
  Aborted,
  FailedWithOutput,
  Timeout,
  CachedFailure,
  UnsupportedSystem,
  LogLimitExceeded,
  NarSizeLimitExceeded,
  NonDeterministic,
}

impl BuildStatusFilter {
  const fn as_db_str(self) -> &'static str {
    match self {
      Self::Pending => "pending",
      Self::Running => "running",
      Self::Succeeded => "succeeded",
      Self::Failed => "failed",
      Self::Cancelled => "cancelled",
      Self::DependencyFailed => "dependency_failed",
      Self::Aborted => "aborted",
      Self::FailedWithOutput => "failed_with_output",
      Self::Timeout => "timeout",
      Self::CachedFailure => "cached_failure",
      Self::UnsupportedSystem => "unsupported_system",
      Self::LogLimitExceeded => "log_limit_exceeded",
      Self::NarSizeLimitExceeded => "nar_size_limit_exceeded",
      Self::NonDeterministic => "non_deterministic",
    }
  }
}

/// Search filters for builds
#[derive(Debug, Clone, Default)]
pub struct BuildSearchFilters {
  pub status:          Option<BuildStatusFilter>,
  pub project_id:      Option<Uuid>,
  pub jobset_id:       Option<Uuid>,
  pub evaluation_id:   Option<Uuid>,
  pub created_after:   Option<chrono::DateTime<chrono::Utc>>,
  pub created_before:  Option<chrono::DateTime<chrono::Utc>>,
  pub min_priority:    Option<i32>,
  pub max_priority:    Option<i32>,
  pub has_substitutes: Option<bool>,
}

/// Search filters for projects
#[derive(Debug, Clone, Default)]
pub struct ProjectSearchFilters {
  pub created_after:  Option<chrono::DateTime<chrono::Utc>>,
  pub created_before: Option<chrono::DateTime<chrono::Utc>>,
  pub has_jobsets:    Option<bool>,
}

/// Search filters for jobsets
#[derive(Debug, Clone, Default)]
pub struct JobsetSearchFilters {
  pub project_id: Option<Uuid>,
  pub enabled:    Option<bool>,
  pub flake_mode: Option<bool>,
}

/// Search filters for evaluations
#[derive(Debug, Clone, Default)]
pub struct EvaluationSearchFilters {
  pub project_id:      Option<Uuid>,
  pub jobset_id:       Option<Uuid>,
  pub has_builds:      Option<bool>,
  pub finished_after:  Option<chrono::DateTime<chrono::Utc>>,
  pub finished_before: Option<chrono::DateTime<chrono::Utc>>,
}

/// Search parameters
#[derive(Debug, Clone)]
pub struct SearchParams {
  pub query:              String,
  pub entities:           Vec<SearchEntity>,
  pub limit:              i64,
  pub offset:             i64,
  pub build_filters:      Option<BuildSearchFilters>,
  pub project_filters:    Option<ProjectSearchFilters>,
  pub jobset_filters:     Option<JobsetSearchFilters>,
  pub evaluation_filters: Option<EvaluationSearchFilters>,
  pub build_sort:         Option<(BuildSortField, SortOrder)>,
  pub project_sort:       Option<(ProjectSortField, SortOrder)>,
}

impl Default for SearchParams {
  fn default() -> Self {
    Self {
      query:              String::new(),
      entities:           vec![SearchEntity::Projects, SearchEntity::Builds],
      limit:              20,
      offset:             0,
      build_filters:      None,
      project_filters:    None,
      jobset_filters:     None,
      evaluation_filters: None,
      build_sort:         None,
      project_sort:       None,
    }
  }
}

/// Search results container
#[derive(Debug, Clone)]
pub struct SearchResults {
  pub projects:          Vec<Project>,
  pub jobsets:           Vec<Jobset>,
  pub evaluations:       Vec<Evaluation>,
  pub builds:            Vec<Build>,
  pub total_projects:    i64,
  pub total_jobsets:     i64,
  pub total_evaluations: i64,
  pub total_builds:      i64,
}

/// A boxed owned SQL parameter; lets us assemble a positional argument list as
/// the dynamic WHERE clause grows.
type Param = Box<dyn ToSql + Sync + Send>;

const fn order_keyword(order: SortOrder) -> &'static str {
  match order {
    SortOrder::Asc => "ASC",
    SortOrder::Desc => "DESC",
  }
}

fn param_refs(params: &[Param]) -> Vec<&(dyn ToSql + Sync)> {
  params.iter().map(|p| &**p as &(dyn ToSql + Sync)).collect()
}

fn like_pattern(query: &str) -> String {
  if query.is_empty() {
    "%".to_string()
  } else {
    format!("%{query}%")
  }
}

// --- row -> model mappers -------------------------------------------------

fn project_from_row(row: &Row) -> Project {
  Project {
    id:             row.get("id"),
    name:           row.get("name"),
    description:    row.get("description"),
    repository_url: row.get("repository_url"),
    created_at:     row.get("created_at"),
    updated_at:     row.get("updated_at"),
  }
}

fn project_from_quick_search_row(
  row: search_q::ProjectQuickSearchRow,
) -> Project {
  Project {
    id:             row.id,
    name:           row.name,
    description:    row.description,
    repository_url: row.repository_url,
    created_at:     row.created_at,
    updated_at:     row.updated_at,
  }
}

fn jobset_from_row(row: &Row) -> Jobset {
  let state: String = row.get("state");
  let trigger_mode: String = row.get("trigger_mode");
  Jobset {
    id:                row.get("id"),
    project_id:        row.get("project_id"),
    name:              row.get("name"),
    nix_expression:    row.get("nix_expression"),
    enabled:           row.get("enabled"),
    flake_mode:        row.get("flake_mode"),
    check_interval:    row.get("check_interval"),
    trigger_mode:      JobsetTriggerMode::from_db_str(&trigger_mode),
    branch:            row.get("branch"),
    scheduling_shares: row.get("scheduling_shares"),
    created_at:        row.get("created_at"),
    updated_at:        row.get("updated_at"),
    state:             JobsetState::from_db_str(&state),
    last_checked_at:   row.get("last_checked_at"),
    keep_nr:           row.get("keep_nr"),
  }
}

fn evaluation_from_row(row: &Row) -> Evaluation {
  let status: String = row.get("status");
  let trigger_kind: String = row.get("trigger_kind");
  Evaluation {
    id:              row.get("id"),
    jobset_id:       row.get("jobset_id"),
    commit_hash:     row.get("commit_hash"),
    evaluation_time: row.get("evaluation_time"),
    status:          EvaluationStatus::from_db_str(&status),
    error_message:   row.get("error_message"),
    inputs_hash:     row.get("inputs_hash"),
    trigger_kind:    EvaluationTriggerKind::from_db_str(&trigger_kind),
    hidden:          row.get("hidden"),
    pr_number:       row.get("pr_number"),
    pr_head_branch:  row.get("pr_head_branch"),
    pr_base_branch:  row.get("pr_base_branch"),
    pr_action:       row.get("pr_action"),
  }
}

fn build_from_row(row: &Row) -> Build {
  let status: String = row.get("status");
  Build {
    id:                         row.get("id"),
    evaluation_id:              row.get("evaluation_id"),
    job_name:                   row.get("job_name"),
    drv_path:                   row.get("drv_path"),
    status:                     BuildStatus::from_db_str(&status),
    started_at:                 row.get("started_at"),
    completed_at:               row.get("completed_at"),
    log_path:                   row.get("log_path"),
    build_output_path:          row.get("build_output_path"),
    error_message:              row.get("error_message"),
    system:                     row.get("system"),
    priority:                   row.get("priority"),
    retry_count:                row.get("retry_count"),
    max_retries:                row.get("max_retries"),
    notification_pending_since: row.get("notification_pending_since"),
    created_at:                 row.get("created_at"),
    outputs:                    row.get("outputs"),
    is_aggregate:               row.get("is_aggregate"),
    constituents:               row.get("constituents"),
    builder_id:                 row.get("builder_id"),
    agent_machine_id:           row.get("agent_machine_id"),
    signed:                     row.get("signed"),
    keep:                       row.get("keep"),
    is_fod:                     row.get("is_fod"),
    fod_hash:                   row.get("fod_hash"),
    meta_description:           row.get("meta_description"),
    meta_license:               row.get("meta_license"),
    meta_homepage:              row.get("meta_homepage"),
    meta_maintainers:           row.get("meta_maintainers"),
    required_features:          row.get("required_features"),
  }
}

fn build_from_quick_search_row(row: search_q::BuildQuickSearchRow) -> Build {
  Build {
    id:                         row.id,
    evaluation_id:              row.evaluation_id,
    job_name:                   row.job_name,
    drv_path:                   row.drv_path,
    status:                     BuildStatus::from_db_str(&row.status),
    started_at:                 row.started_at,
    completed_at:               row.completed_at,
    log_path:                   row.log_path,
    build_output_path:          row.build_output_path,
    error_message:              row.error_message,
    system:                     row.system,
    priority:                   row.priority,
    retry_count:                row.retry_count,
    max_retries:                row.max_retries,
    notification_pending_since: row.notification_pending_since,
    created_at:                 row.created_at,
    outputs:                    row.outputs,
    is_aggregate:               row.is_aggregate,
    constituents:               row.constituents,
    builder_id:                 row.builder_id,
    agent_machine_id:           row.agent_machine_id,
    signed:                     row.signed,
    keep:                       row.keep,
    is_fod:                     row.is_fod,
    fod_hash:                   row.fod_hash,
    meta_description:           row.meta_description,
    meta_license:               row.meta_license,
    meta_homepage:              row.meta_homepage,
    meta_maintainers:           row.meta_maintainers,
    required_features:          row.required_features,
  }
}

/// Execute a comprehensive search across all entities
///
/// # Errors
///
/// Returns error if database query fails.
pub async fn search(
  pool: &PgPool,
  params: &SearchParams,
) -> Result<SearchResults> {
  let mut results = SearchResults {
    projects:          vec![],
    jobsets:           vec![],
    evaluations:       vec![],
    builds:            vec![],
    total_projects:    0,
    total_jobsets:     0,
    total_evaluations: 0,
    total_builds:      0,
  };

  for entity in &params.entities {
    match entity {
      SearchEntity::Projects => {
        let (projects, total) = search_projects(pool, params).await?;
        results.projects = projects;
        results.total_projects = total;
      },
      SearchEntity::Jobsets => {
        let (jobsets, total) = search_jobsets(pool, params).await?;
        results.jobsets = jobsets;
        results.total_jobsets = total;
      },
      SearchEntity::Evaluations => {
        let (evaluations, total) = search_evaluations(pool, params).await?;
        results.evaluations = evaluations;
        results.total_evaluations = total;
      },
      SearchEntity::Builds => {
        let (builds, total) = search_builds(pool, params).await?;
        results.builds = builds;
        results.total_builds = total;
      },
    }
  }

  Ok(results)
}

/// Search projects with filters
async fn search_projects(
  pool: &PgPool,
  params: &SearchParams,
) -> Result<(Vec<Project>, i64)> {
  let pattern = like_pattern(&params.query);

  let mut sql = String::from(
    "SELECT * FROM projects WHERE (name ILIKE $1 OR description ILIKE $1)",
  );
  let mut args: Vec<Param> = vec![Box::new(pattern.clone())];

  if let Some(filters) = &params.project_filters {
    if let Some(after) = filters.created_after {
      args.push(Box::new(after));
      let _ = write!(sql, " AND created_at >= ${}", args.len());
    }
    if let Some(before) = filters.created_before {
      args.push(Box::new(before));
      let _ = write!(sql, " AND created_at <= ${}", args.len());
    }
    if let Some(has_jobsets) = filters.has_jobsets {
      sql.push_str(if has_jobsets {
        " AND EXISTS (SELECT 1 FROM jobsets WHERE jobsets.project_id = \
         projects.id)"
      } else {
        " AND NOT EXISTS (SELECT 1 FROM jobsets WHERE jobsets.project_id = \
         projects.id)"
      });
    }
  }

  let client = pool.get().await?;

  let total: i64 = if pattern == "%" {
    client
      .query_one("SELECT COUNT(*) FROM projects", &[])
      .await?
      .get(0)
  } else {
    client
      .query_one(
        "SELECT COUNT(*) FROM projects WHERE name ILIKE $1 OR description \
         ILIKE $1",
        &[&pattern],
      )
      .await?
      .get(0)
  };

  sql.push_str(" ORDER BY ");
  if let Some((field, order)) = &params.project_sort {
    let field_str = match field {
      ProjectSortField::Name => "name",
      ProjectSortField::CreatedAt => "created_at",
      ProjectSortField::LastEvaluation => "last_evaluation_at",
    };
    let _ = write!(sql, "{field_str} {}", order_keyword(*order));
  } else {
    sql.push_str("name ASC");
  }

  args.push(Box::new(params.limit));
  let _ = write!(sql, " LIMIT ${}", args.len());
  args.push(Box::new(params.offset));
  let _ = write!(sql, " OFFSET ${}", args.len());

  let rows = client.query(&sql, &param_refs(&args)).await?;
  Ok((rows.iter().map(project_from_row).collect(), total))
}

/// Search jobsets with filters
async fn search_jobsets(
  pool: &PgPool,
  params: &SearchParams,
) -> Result<(Vec<Jobset>, i64)> {
  let pattern = like_pattern(&params.query);

  let mut where_sql = String::from(" WHERE name ILIKE $1");
  let mut args: Vec<Param> = vec![Box::new(pattern)];

  if let Some(filters) = &params.jobset_filters {
    if let Some(project_id) = filters.project_id {
      args.push(Box::new(project_id));
      let _ = write!(where_sql, " AND project_id = ${}", args.len());
    }
    if let Some(enabled) = filters.enabled {
      args.push(Box::new(enabled));
      let _ = write!(where_sql, " AND enabled = ${}", args.len());
    }
    if let Some(flake_mode) = filters.flake_mode {
      args.push(Box::new(flake_mode));
      let _ = write!(where_sql, " AND flake_mode = ${}", args.len());
    }
  }

  let client = pool.get().await?;

  let count_sql = format!("SELECT COUNT(*) FROM jobsets{where_sql}");
  let total: i64 = client
    .query_one(&count_sql, &param_refs(&args))
    .await?
    .get(0);

  let mut sql = format!("SELECT * FROM jobsets{where_sql} ORDER BY name ASC");
  args.push(Box::new(params.limit));
  let _ = write!(sql, " LIMIT ${}", args.len());
  args.push(Box::new(params.offset));
  let _ = write!(sql, " OFFSET ${}", args.len());

  let rows = client.query(&sql, &param_refs(&args)).await?;
  Ok((rows.iter().map(jobset_from_row).collect(), total))
}

/// Search evaluations with filters
async fn search_evaluations(
  pool: &PgPool,
  params: &SearchParams,
) -> Result<(Vec<Evaluation>, i64)> {
  let mut where_sql = String::from(" WHERE 1=1");
  let mut args: Vec<Param> = vec![];

  if let Some(filters) = &params.evaluation_filters {
    if let Some(project_id) = filters.project_id {
      args.push(Box::new(project_id));
      let _ = write!(where_sql, " AND project_id = ${}", args.len());
    }
    if let Some(jobset_id) = filters.jobset_id {
      args.push(Box::new(jobset_id));
      let _ = write!(where_sql, " AND jobset_id = ${}", args.len());
    }
    if let Some(has_builds) = filters.has_builds {
      where_sql.push_str(if has_builds {
        " AND EXISTS (SELECT 1 FROM builds WHERE builds.evaluation_id = \
         evaluations.id)"
      } else {
        " AND NOT EXISTS (SELECT 1 FROM builds WHERE builds.evaluation_id = \
         evaluations.id)"
      });
    }
    if let Some(after) = filters.finished_after {
      args.push(Box::new(after));
      let _ = write!(where_sql, " AND finished_at >= ${}", args.len());
    }
    if let Some(before) = filters.finished_before {
      args.push(Box::new(before));
      let _ = write!(where_sql, " AND finished_at <= ${}", args.len());
    }
  }

  let client = pool.get().await?;

  let count_sql = format!("SELECT COUNT(*) FROM evaluations{where_sql}");
  let total: i64 = client
    .query_one(&count_sql, &param_refs(&args))
    .await?
    .get(0);

  let mut sql =
    format!("SELECT * FROM evaluations{where_sql} ORDER BY created_at DESC");
  args.push(Box::new(params.limit));
  let _ = write!(sql, " LIMIT ${}", args.len());
  args.push(Box::new(params.offset));
  let _ = write!(sql, " OFFSET ${}", args.len());

  let rows = client.query(&sql, &param_refs(&args)).await?;
  Ok((rows.iter().map(evaluation_from_row).collect(), total))
}

/// Search builds with advanced filters
async fn search_builds(
  pool: &PgPool,
  params: &SearchParams,
) -> Result<(Vec<Build>, i64)> {
  let pattern = like_pattern(&params.query);

  let mut where_sql =
    String::from(" WHERE (job_name ILIKE $1 OR drv_path ILIKE $1)");
  let mut args: Vec<Param> = vec![Box::new(pattern)];

  if let Some(filters) = &params.build_filters {
    if let Some(status) = filters.status {
      args.push(Box::new(status.as_db_str()));
      let _ = write!(where_sql, " AND status = ${}", args.len());
    }
    if let Some(project_id) = filters.project_id {
      args.push(Box::new(project_id));
      let _ = write!(where_sql, " AND project_id = ${}", args.len());
    }
    if let Some(jobset_id) = filters.jobset_id {
      args.push(Box::new(jobset_id));
      let _ = write!(where_sql, " AND jobset_id = ${}", args.len());
    }
    if let Some(evaluation_id) = filters.evaluation_id {
      args.push(Box::new(evaluation_id));
      let _ = write!(where_sql, " AND evaluation_id = ${}", args.len());
    }
    if let Some(after) = filters.created_after {
      args.push(Box::new(after));
      let _ = write!(where_sql, " AND created_at >= ${}", args.len());
    }
    if let Some(before) = filters.created_before {
      args.push(Box::new(before));
      let _ = write!(where_sql, " AND created_at <= ${}", args.len());
    }
    if let Some(min) = filters.min_priority {
      args.push(Box::new(min));
      let _ = write!(where_sql, " AND priority >= ${}", args.len());
    }
    if let Some(max) = filters.max_priority {
      args.push(Box::new(max));
      let _ = write!(where_sql, " AND priority <= ${}", args.len());
    }
    if let Some(has) = filters.has_substitutes {
      args.push(Box::new(has));
      let _ = write!(where_sql, " AND has_substitutes = ${}", args.len());
    }
  }

  let client = pool.get().await?;

  let count_sql = format!("SELECT COUNT(*) FROM builds{where_sql}");
  let total: i64 = client
    .query_one(&count_sql, &param_refs(&args))
    .await?
    .get(0);

  let mut sql = format!("SELECT * FROM builds{where_sql} ORDER BY ");
  if let Some((field, order)) = &params.build_sort {
    let field_str = match field {
      BuildSortField::CreatedAt => "created_at",
      BuildSortField::JobName => "job_name",
      BuildSortField::Status => "status",
      BuildSortField::Priority => "priority",
    };
    let _ = write!(sql, "{field_str} {}", order_keyword(*order));
  } else {
    sql.push_str("created_at DESC");
  }

  args.push(Box::new(params.limit));
  let _ = write!(sql, " LIMIT ${}", args.len());
  args.push(Box::new(params.offset));
  let _ = write!(sql, " OFFSET ${}", args.len());

  let rows = client.query(&sql, &param_refs(&args)).await?;
  Ok((rows.iter().map(build_from_row).collect(), total))
}

/// Quick search - simple text search across entities
///
/// # Errors
///
/// Returns error if database query fails.
pub async fn quick_search(
  pool: &PgPool,
  query: &str,
  limit: i64,
) -> Result<(Vec<Project>, Vec<Build>)> {
  let pattern = format!("%{query}%");
  let client = pool.get().await?;

  let project_rows = search_q::quick_projects()
    .bind(&client, &pattern, &limit)
    .all()
    .await?;
  let build_rows = search_q::quick_builds()
    .bind(&client, &pattern, &limit)
    .all()
    .await?;

  Ok((
    project_rows
      .into_iter()
      .map(project_from_quick_search_row)
      .collect(),
    build_rows
      .into_iter()
      .map(build_from_quick_search_row)
      .collect(),
  ))
}
