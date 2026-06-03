// This file was generated with `clorinde`. Do not modify.

#[derive(Debug)]
pub struct QuickProjectsParams<T1: crate::StringSql> {
    pub pattern: T1,
    pub limit: i64,
}
#[derive(Debug)]
pub struct QuickBuildsParams<T1: crate::StringSql> {
    pub pattern: T1,
    pub limit: i64,
}
#[derive(Debug, Clone, PartialEq)]
pub struct ProjectQuickSearchRow {
    pub id: uuid::Uuid,
    pub name: String,
    pub description: Option<String>,
    pub repository_url: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
pub struct ProjectQuickSearchRowBorrowed<'a> {
    pub id: uuid::Uuid,
    pub name: &'a str,
    pub description: Option<&'a str>,
    pub repository_url: &'a str,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
impl<'a> From<ProjectQuickSearchRowBorrowed<'a>> for ProjectQuickSearchRow {
    fn from(
        ProjectQuickSearchRowBorrowed {
            id,
            name,
            description,
            repository_url,
            created_at,
            updated_at,
        }: ProjectQuickSearchRowBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            name: name.into(),
            description: description.map(|v| v.into()),
            repository_url: repository_url.into(),
            created_at,
            updated_at,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct BuildQuickSearchRow {
    pub id: uuid::Uuid,
    pub evaluation_id: uuid::Uuid,
    pub job_name: String,
    pub drv_path: String,
    pub status: String,
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    pub log_path: Option<String>,
    pub build_output_path: Option<String>,
    pub error_message: Option<String>,
    pub priority: i32,
    pub retry_count: i32,
    pub max_retries: i32,
    pub notification_pending_since: Option<chrono::DateTime<chrono::Utc>>,
    pub outputs: Option<serde_json::Value>,
    pub is_aggregate: bool,
    pub constituents: Option<serde_json::Value>,
    pub builder_id: Option<uuid::Uuid>,
    pub signed: bool,
    pub system: Option<String>,
    pub keep: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub is_fod: bool,
    pub fod_hash: Option<String>,
    pub meta_description: Option<String>,
    pub meta_license: Option<String>,
    pub meta_homepage: Option<String>,
    pub meta_maintainers: Option<String>,
    pub required_features: Vec<String>,
    pub agent_machine_id: Option<uuid::Uuid>,
    pub started_notified_at: Option<chrono::DateTime<chrono::Utc>>,
}
pub struct BuildQuickSearchRowBorrowed<'a> {
    pub id: uuid::Uuid,
    pub evaluation_id: uuid::Uuid,
    pub job_name: &'a str,
    pub drv_path: &'a str,
    pub status: &'a str,
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    pub log_path: Option<&'a str>,
    pub build_output_path: Option<&'a str>,
    pub error_message: Option<&'a str>,
    pub priority: i32,
    pub retry_count: i32,
    pub max_retries: i32,
    pub notification_pending_since: Option<chrono::DateTime<chrono::Utc>>,
    pub outputs: Option<postgres_types::Json<&'a serde_json::value::RawValue>>,
    pub is_aggregate: bool,
    pub constituents: Option<postgres_types::Json<&'a serde_json::value::RawValue>>,
    pub builder_id: Option<uuid::Uuid>,
    pub signed: bool,
    pub system: Option<&'a str>,
    pub keep: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub is_fod: bool,
    pub fod_hash: Option<&'a str>,
    pub meta_description: Option<&'a str>,
    pub meta_license: Option<&'a str>,
    pub meta_homepage: Option<&'a str>,
    pub meta_maintainers: Option<&'a str>,
    pub required_features: crate::ArrayIterator<'a, &'a str>,
    pub agent_machine_id: Option<uuid::Uuid>,
    pub started_notified_at: Option<chrono::DateTime<chrono::Utc>>,
}
impl<'a> From<BuildQuickSearchRowBorrowed<'a>> for BuildQuickSearchRow {
    fn from(
        BuildQuickSearchRowBorrowed {
            id,
            evaluation_id,
            job_name,
            drv_path,
            status,
            started_at,
            completed_at,
            log_path,
            build_output_path,
            error_message,
            priority,
            retry_count,
            max_retries,
            notification_pending_since,
            outputs,
            is_aggregate,
            constituents,
            builder_id,
            signed,
            system,
            keep,
            created_at,
            is_fod,
            fod_hash,
            meta_description,
            meta_license,
            meta_homepage,
            meta_maintainers,
            required_features,
            agent_machine_id,
            started_notified_at,
        }: BuildQuickSearchRowBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            evaluation_id,
            job_name: job_name.into(),
            drv_path: drv_path.into(),
            status: status.into(),
            started_at,
            completed_at,
            log_path: log_path.map(|v| v.into()),
            build_output_path: build_output_path.map(|v| v.into()),
            error_message: error_message.map(|v| v.into()),
            priority,
            retry_count,
            max_retries,
            notification_pending_since,
            outputs: outputs.map(|v| serde_json::from_str(v.0.get()).unwrap()),
            is_aggregate,
            constituents: constituents.map(|v| serde_json::from_str(v.0.get()).unwrap()),
            builder_id,
            signed,
            system: system.map(|v| v.into()),
            keep,
            created_at,
            is_fod,
            fod_hash: fod_hash.map(|v| v.into()),
            meta_description: meta_description.map(|v| v.into()),
            meta_license: meta_license.map(|v| v.into()),
            meta_homepage: meta_homepage.map(|v| v.into()),
            meta_maintainers: meta_maintainers.map(|v| v.into()),
            required_features: required_features.map(|v| v.into()).collect(),
            agent_machine_id,
            started_notified_at,
        }
    }
}
use crate::client::async_::GenericClient;
use futures::{self, StreamExt, TryStreamExt};
pub struct ProjectQuickSearchRowQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor:
        fn(&tokio_postgres::Row) -> Result<ProjectQuickSearchRowBorrowed, tokio_postgres::Error>,
    mapper: fn(ProjectQuickSearchRowBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> ProjectQuickSearchRowQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(ProjectQuickSearchRowBorrowed) -> R,
    ) -> ProjectQuickSearchRowQuery<'c, 'a, 's, C, R, N> {
        ProjectQuickSearchRowQuery {
            client: self.client,
            params: self.params,
            query: self.query,
            cached: self.cached,
            extractor: self.extractor,
            mapper,
        }
    }
    pub async fn one(self) -> Result<T, tokio_postgres::Error> {
        let row =
            crate::client::async_::one(self.client, self.query, &self.params, self.cached).await?;
        Ok((self.mapper)((self.extractor)(&row)?))
    }
    pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
        self.iter().await?.try_collect().await
    }
    pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
        let opt_row =
            crate::client::async_::opt(self.client, self.query, &self.params, self.cached).await?;
        Ok(opt_row
            .map(|row| {
                let extracted = (self.extractor)(&row)?;
                Ok((self.mapper)(extracted))
            })
            .transpose()?)
    }
    pub async fn iter(
        self,
    ) -> Result<
        impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'c,
        tokio_postgres::Error,
    > {
        let stream = crate::client::async_::raw(
            self.client,
            self.query,
            crate::slice_iter(&self.params),
            self.cached,
        )
        .await?;
        let mapped = stream
            .map(move |res| {
                res.and_then(|row| {
                    let extracted = (self.extractor)(&row)?;
                    Ok((self.mapper)(extracted))
                })
            })
            .into_stream();
        Ok(mapped)
    }
}
pub struct BuildQuickSearchRowQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor:
        fn(&tokio_postgres::Row) -> Result<BuildQuickSearchRowBorrowed, tokio_postgres::Error>,
    mapper: fn(BuildQuickSearchRowBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> BuildQuickSearchRowQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(BuildQuickSearchRowBorrowed) -> R,
    ) -> BuildQuickSearchRowQuery<'c, 'a, 's, C, R, N> {
        BuildQuickSearchRowQuery {
            client: self.client,
            params: self.params,
            query: self.query,
            cached: self.cached,
            extractor: self.extractor,
            mapper,
        }
    }
    pub async fn one(self) -> Result<T, tokio_postgres::Error> {
        let row =
            crate::client::async_::one(self.client, self.query, &self.params, self.cached).await?;
        Ok((self.mapper)((self.extractor)(&row)?))
    }
    pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
        self.iter().await?.try_collect().await
    }
    pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
        let opt_row =
            crate::client::async_::opt(self.client, self.query, &self.params, self.cached).await?;
        Ok(opt_row
            .map(|row| {
                let extracted = (self.extractor)(&row)?;
                Ok((self.mapper)(extracted))
            })
            .transpose()?)
    }
    pub async fn iter(
        self,
    ) -> Result<
        impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'c,
        tokio_postgres::Error,
    > {
        let stream = crate::client::async_::raw(
            self.client,
            self.query,
            crate::slice_iter(&self.params),
            self.cached,
        )
        .await?;
        let mapped = stream
            .map(move |res| {
                res.and_then(|row| {
                    let extracted = (self.extractor)(&row)?;
                    Ok((self.mapper)(extracted))
                })
            })
            .into_stream();
        Ok(mapped)
    }
}
pub struct QuickProjectsStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn quick_projects() -> QuickProjectsStmt {
    QuickProjectsStmt(
        "SELECT * FROM projects WHERE name ILIKE $1 OR description ILIKE $1 ORDER BY name LIMIT $2",
        None,
    )
}
impl QuickProjectsStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>(
        &'s self,
        client: &'c C,
        pattern: &'a T1,
        limit: &'a i64,
    ) -> ProjectQuickSearchRowQuery<'c, 'a, 's, C, ProjectQuickSearchRow, 2> {
        ProjectQuickSearchRowQuery {
            client,
            params: [pattern, limit],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |
                row: &tokio_postgres::Row,
            | -> Result<ProjectQuickSearchRowBorrowed, tokio_postgres::Error> {
                Ok(ProjectQuickSearchRowBorrowed {
                    id: row.try_get(0)?,
                    name: row.try_get(1)?,
                    description: row.try_get(2)?,
                    repository_url: row.try_get(3)?,
                    created_at: row.try_get(4)?,
                    updated_at: row.try_get(5)?,
                })
            },
            mapper: |it| ProjectQuickSearchRow::from(it),
        }
    }
}
impl<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>
    crate::client::async_::Params<
        'c,
        'a,
        's,
        QuickProjectsParams<T1>,
        ProjectQuickSearchRowQuery<'c, 'a, 's, C, ProjectQuickSearchRow, 2>,
        C,
    > for QuickProjectsStmt
{
    fn params(
        &'s self,
        client: &'c C,
        params: &'a QuickProjectsParams<T1>,
    ) -> ProjectQuickSearchRowQuery<'c, 'a, 's, C, ProjectQuickSearchRow, 2> {
        self.bind(client, &params.pattern, &params.limit)
    }
}
pub struct QuickBuildsStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn quick_builds() -> QuickBuildsStmt {
    QuickBuildsStmt(
        "SELECT * FROM builds WHERE job_name ILIKE $1 OR drv_path ILIKE $1 ORDER BY created_at DESC LIMIT $2",
        None,
    )
}
impl QuickBuildsStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>(
        &'s self,
        client: &'c C,
        pattern: &'a T1,
        limit: &'a i64,
    ) -> BuildQuickSearchRowQuery<'c, 'a, 's, C, BuildQuickSearchRow, 2> {
        BuildQuickSearchRowQuery {
            client,
            params: [pattern, limit],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |
                row: &tokio_postgres::Row,
            | -> Result<BuildQuickSearchRowBorrowed, tokio_postgres::Error> {
                Ok(BuildQuickSearchRowBorrowed {
                    id: row.try_get(0)?,
                    evaluation_id: row.try_get(1)?,
                    job_name: row.try_get(2)?,
                    drv_path: row.try_get(3)?,
                    status: row.try_get(4)?,
                    started_at: row.try_get(5)?,
                    completed_at: row.try_get(6)?,
                    log_path: row.try_get(7)?,
                    build_output_path: row.try_get(8)?,
                    error_message: row.try_get(9)?,
                    priority: row.try_get(10)?,
                    retry_count: row.try_get(11)?,
                    max_retries: row.try_get(12)?,
                    notification_pending_since: row.try_get(13)?,
                    outputs: row.try_get(14)?,
                    is_aggregate: row.try_get(15)?,
                    constituents: row.try_get(16)?,
                    builder_id: row.try_get(17)?,
                    signed: row.try_get(18)?,
                    system: row.try_get(19)?,
                    keep: row.try_get(20)?,
                    created_at: row.try_get(21)?,
                    is_fod: row.try_get(22)?,
                    fod_hash: row.try_get(23)?,
                    meta_description: row.try_get(24)?,
                    meta_license: row.try_get(25)?,
                    meta_homepage: row.try_get(26)?,
                    meta_maintainers: row.try_get(27)?,
                    required_features: row.try_get(28)?,
                    agent_machine_id: row.try_get(29)?,
                    started_notified_at: row.try_get(30)?,
                })
            },
            mapper: |it| BuildQuickSearchRow::from(it),
        }
    }
}
impl<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>
    crate::client::async_::Params<
        'c,
        'a,
        's,
        QuickBuildsParams<T1>,
        BuildQuickSearchRowQuery<'c, 'a, 's, C, BuildQuickSearchRow, 2>,
        C,
    > for QuickBuildsStmt
{
    fn params(
        &'s self,
        client: &'c C,
        params: &'a QuickBuildsParams<T1>,
    ) -> BuildQuickSearchRowQuery<'c, 'a, 's, C, BuildQuickSearchRow, 2> {
        self.bind(client, &params.pattern, &params.limit)
    }
}
