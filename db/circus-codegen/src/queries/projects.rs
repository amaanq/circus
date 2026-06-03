// This file was generated with `clorinde`. Do not modify.

#[derive(Debug)]
pub struct CreateParams<T1: crate::StringSql, T2: crate::StringSql, T3: crate::StringSql> {
    pub name: T1,
    pub description: Option<T2>,
    pub repository_url: T3,
}
#[derive(Clone, Copy, Debug)]
pub struct ListParams {
    pub limit: i64,
    pub offset: i64,
}
#[derive(Debug)]
pub struct UpdateParams<T1: crate::StringSql, T2: crate::StringSql, T3: crate::StringSql> {
    pub name: T1,
    pub description: Option<T2>,
    pub repository_url: T3,
    pub id: uuid::Uuid,
}
#[derive(Debug)]
pub struct UpsertParams<T1: crate::StringSql, T2: crate::StringSql, T3: crate::StringSql> {
    pub name: T1,
    pub description: Option<T2>,
    pub repository_url: T3,
}
#[derive(Debug, Clone, PartialEq)]
pub struct ProjectRow {
    pub id: uuid::Uuid,
    pub name: String,
    pub description: Option<String>,
    pub repository_url: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
pub struct ProjectRowBorrowed<'a> {
    pub id: uuid::Uuid,
    pub name: &'a str,
    pub description: Option<&'a str>,
    pub repository_url: &'a str,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
impl<'a> From<ProjectRowBorrowed<'a>> for ProjectRow {
    fn from(
        ProjectRowBorrowed {
            id,
            name,
            description,
            repository_url,
            created_at,
            updated_at,
        }: ProjectRowBorrowed<'a>,
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
use crate::client::async_::GenericClient;
use futures::{self, StreamExt, TryStreamExt};
pub struct ProjectRowQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<ProjectRowBorrowed, tokio_postgres::Error>,
    mapper: fn(ProjectRowBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> ProjectRowQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(ProjectRowBorrowed) -> R,
    ) -> ProjectRowQuery<'c, 'a, 's, C, R, N> {
        ProjectRowQuery {
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
pub struct I64Query<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<i64, tokio_postgres::Error>,
    mapper: fn(i64) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> I64Query<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(self, mapper: fn(i64) -> R) -> I64Query<'c, 'a, 's, C, R, N> {
        I64Query {
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
pub struct CreateStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn create() -> CreateStmt {
    CreateStmt(
        "INSERT INTO projects (name, description, repository_url) VALUES ($1, $2, $3) RETURNING *",
        None,
    )
}
impl CreateStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub fn bind<
        'c,
        'a,
        's,
        C: GenericClient,
        T1: crate::StringSql,
        T2: crate::StringSql,
        T3: crate::StringSql,
    >(
        &'s self,
        client: &'c C,
        name: &'a T1,
        description: &'a Option<T2>,
        repository_url: &'a T3,
    ) -> ProjectRowQuery<'c, 'a, 's, C, ProjectRow, 3> {
        ProjectRowQuery {
            client,
            params: [name, description, repository_url],
            query: self.0,
            cached: self.1.as_ref(),
            extractor:
                |row: &tokio_postgres::Row| -> Result<ProjectRowBorrowed, tokio_postgres::Error> {
                    Ok(ProjectRowBorrowed {
                        id: row.try_get(0)?,
                        name: row.try_get(1)?,
                        description: row.try_get(2)?,
                        repository_url: row.try_get(3)?,
                        created_at: row.try_get(4)?,
                        updated_at: row.try_get(5)?,
                    })
                },
            mapper: |it| ProjectRow::from(it),
        }
    }
}
impl<'c, 'a, 's, C: GenericClient, T1: crate::StringSql, T2: crate::StringSql, T3: crate::StringSql>
    crate::client::async_::Params<
        'c,
        'a,
        's,
        CreateParams<T1, T2, T3>,
        ProjectRowQuery<'c, 'a, 's, C, ProjectRow, 3>,
        C,
    > for CreateStmt
{
    fn params(
        &'s self,
        client: &'c C,
        params: &'a CreateParams<T1, T2, T3>,
    ) -> ProjectRowQuery<'c, 'a, 's, C, ProjectRow, 3> {
        self.bind(
            client,
            &params.name,
            &params.description,
            &params.repository_url,
        )
    }
}
pub struct GetStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn get() -> GetStmt {
    GetStmt("SELECT * FROM projects WHERE id = $1", None)
}
impl GetStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub fn bind<'c, 'a, 's, C: GenericClient>(
        &'s self,
        client: &'c C,
        id: &'a uuid::Uuid,
    ) -> ProjectRowQuery<'c, 'a, 's, C, ProjectRow, 1> {
        ProjectRowQuery {
            client,
            params: [id],
            query: self.0,
            cached: self.1.as_ref(),
            extractor:
                |row: &tokio_postgres::Row| -> Result<ProjectRowBorrowed, tokio_postgres::Error> {
                    Ok(ProjectRowBorrowed {
                        id: row.try_get(0)?,
                        name: row.try_get(1)?,
                        description: row.try_get(2)?,
                        repository_url: row.try_get(3)?,
                        created_at: row.try_get(4)?,
                        updated_at: row.try_get(5)?,
                    })
                },
            mapper: |it| ProjectRow::from(it),
        }
    }
}
pub struct GetByNameStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn get_by_name() -> GetByNameStmt {
    GetByNameStmt("SELECT * FROM projects WHERE name = $1", None)
}
impl GetByNameStmt {
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
        name: &'a T1,
    ) -> ProjectRowQuery<'c, 'a, 's, C, ProjectRow, 1> {
        ProjectRowQuery {
            client,
            params: [name],
            query: self.0,
            cached: self.1.as_ref(),
            extractor:
                |row: &tokio_postgres::Row| -> Result<ProjectRowBorrowed, tokio_postgres::Error> {
                    Ok(ProjectRowBorrowed {
                        id: row.try_get(0)?,
                        name: row.try_get(1)?,
                        description: row.try_get(2)?,
                        repository_url: row.try_get(3)?,
                        created_at: row.try_get(4)?,
                        updated_at: row.try_get(5)?,
                    })
                },
            mapper: |it| ProjectRow::from(it),
        }
    }
}
pub struct ListStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn list() -> ListStmt {
    ListStmt(
        "SELECT * FROM projects ORDER BY created_at DESC LIMIT $1 OFFSET $2",
        None,
    )
}
impl ListStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub fn bind<'c, 'a, 's, C: GenericClient>(
        &'s self,
        client: &'c C,
        limit: &'a i64,
        offset: &'a i64,
    ) -> ProjectRowQuery<'c, 'a, 's, C, ProjectRow, 2> {
        ProjectRowQuery {
            client,
            params: [limit, offset],
            query: self.0,
            cached: self.1.as_ref(),
            extractor:
                |row: &tokio_postgres::Row| -> Result<ProjectRowBorrowed, tokio_postgres::Error> {
                    Ok(ProjectRowBorrowed {
                        id: row.try_get(0)?,
                        name: row.try_get(1)?,
                        description: row.try_get(2)?,
                        repository_url: row.try_get(3)?,
                        created_at: row.try_get(4)?,
                        updated_at: row.try_get(5)?,
                    })
                },
            mapper: |it| ProjectRow::from(it),
        }
    }
}
impl<'c, 'a, 's, C: GenericClient>
    crate::client::async_::Params<
        'c,
        'a,
        's,
        ListParams,
        ProjectRowQuery<'c, 'a, 's, C, ProjectRow, 2>,
        C,
    > for ListStmt
{
    fn params(
        &'s self,
        client: &'c C,
        params: &'a ListParams,
    ) -> ProjectRowQuery<'c, 'a, 's, C, ProjectRow, 2> {
        self.bind(client, &params.limit, &params.offset)
    }
}
pub struct CountStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn count() -> CountStmt {
    CountStmt("SELECT COUNT(*) FROM projects", None)
}
impl CountStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub fn bind<'c, 'a, 's, C: GenericClient>(
        &'s self,
        client: &'c C,
    ) -> I64Query<'c, 'a, 's, C, i64, 0> {
        I64Query {
            client,
            params: [],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |row| Ok(row.try_get(0)?),
            mapper: |it| it,
        }
    }
}
pub struct UpdateStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn update() -> UpdateStmt {
    UpdateStmt(
        "UPDATE projects SET name = $1, description = $2, repository_url = $3 WHERE id = $4 RETURNING *",
        None,
    )
}
impl UpdateStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub fn bind<
        'c,
        'a,
        's,
        C: GenericClient,
        T1: crate::StringSql,
        T2: crate::StringSql,
        T3: crate::StringSql,
    >(
        &'s self,
        client: &'c C,
        name: &'a T1,
        description: &'a Option<T2>,
        repository_url: &'a T3,
        id: &'a uuid::Uuid,
    ) -> ProjectRowQuery<'c, 'a, 's, C, ProjectRow, 4> {
        ProjectRowQuery {
            client,
            params: [name, description, repository_url, id],
            query: self.0,
            cached: self.1.as_ref(),
            extractor:
                |row: &tokio_postgres::Row| -> Result<ProjectRowBorrowed, tokio_postgres::Error> {
                    Ok(ProjectRowBorrowed {
                        id: row.try_get(0)?,
                        name: row.try_get(1)?,
                        description: row.try_get(2)?,
                        repository_url: row.try_get(3)?,
                        created_at: row.try_get(4)?,
                        updated_at: row.try_get(5)?,
                    })
                },
            mapper: |it| ProjectRow::from(it),
        }
    }
}
impl<'c, 'a, 's, C: GenericClient, T1: crate::StringSql, T2: crate::StringSql, T3: crate::StringSql>
    crate::client::async_::Params<
        'c,
        'a,
        's,
        UpdateParams<T1, T2, T3>,
        ProjectRowQuery<'c, 'a, 's, C, ProjectRow, 4>,
        C,
    > for UpdateStmt
{
    fn params(
        &'s self,
        client: &'c C,
        params: &'a UpdateParams<T1, T2, T3>,
    ) -> ProjectRowQuery<'c, 'a, 's, C, ProjectRow, 4> {
        self.bind(
            client,
            &params.name,
            &params.description,
            &params.repository_url,
            &params.id,
        )
    }
}
pub struct UpsertStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn upsert() -> UpsertStmt {
    UpsertStmt(
        "INSERT INTO projects (name, description, repository_url) VALUES ($1, $2, $3) ON CONFLICT (name) DO UPDATE SET description = EXCLUDED.description, repository_url = EXCLUDED.repository_url RETURNING *",
        None,
    )
}
impl UpsertStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub fn bind<
        'c,
        'a,
        's,
        C: GenericClient,
        T1: crate::StringSql,
        T2: crate::StringSql,
        T3: crate::StringSql,
    >(
        &'s self,
        client: &'c C,
        name: &'a T1,
        description: &'a Option<T2>,
        repository_url: &'a T3,
    ) -> ProjectRowQuery<'c, 'a, 's, C, ProjectRow, 3> {
        ProjectRowQuery {
            client,
            params: [name, description, repository_url],
            query: self.0,
            cached: self.1.as_ref(),
            extractor:
                |row: &tokio_postgres::Row| -> Result<ProjectRowBorrowed, tokio_postgres::Error> {
                    Ok(ProjectRowBorrowed {
                        id: row.try_get(0)?,
                        name: row.try_get(1)?,
                        description: row.try_get(2)?,
                        repository_url: row.try_get(3)?,
                        created_at: row.try_get(4)?,
                        updated_at: row.try_get(5)?,
                    })
                },
            mapper: |it| ProjectRow::from(it),
        }
    }
}
impl<'c, 'a, 's, C: GenericClient, T1: crate::StringSql, T2: crate::StringSql, T3: crate::StringSql>
    crate::client::async_::Params<
        'c,
        'a,
        's,
        UpsertParams<T1, T2, T3>,
        ProjectRowQuery<'c, 'a, 's, C, ProjectRow, 3>,
        C,
    > for UpsertStmt
{
    fn params(
        &'s self,
        client: &'c C,
        params: &'a UpsertParams<T1, T2, T3>,
    ) -> ProjectRowQuery<'c, 'a, 's, C, ProjectRow, 3> {
        self.bind(
            client,
            &params.name,
            &params.description,
            &params.repository_url,
        )
    }
}
pub struct ListWithoutActiveJobsetsStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn list_without_active_jobsets() -> ListWithoutActiveJobsetsStmt {
    ListWithoutActiveJobsetsStmt(
        "SELECT p.* FROM projects p WHERE NOT EXISTS (SELECT 1 FROM jobsets j WHERE j.project_id = p.id)",
        None,
    )
}
impl ListWithoutActiveJobsetsStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub fn bind<'c, 'a, 's, C: GenericClient>(
        &'s self,
        client: &'c C,
    ) -> ProjectRowQuery<'c, 'a, 's, C, ProjectRow, 0> {
        ProjectRowQuery {
            client,
            params: [],
            query: self.0,
            cached: self.1.as_ref(),
            extractor:
                |row: &tokio_postgres::Row| -> Result<ProjectRowBorrowed, tokio_postgres::Error> {
                    Ok(ProjectRowBorrowed {
                        id: row.try_get(0)?,
                        name: row.try_get(1)?,
                        description: row.try_get(2)?,
                        repository_url: row.try_get(3)?,
                        created_at: row.try_get(4)?,
                        updated_at: row.try_get(5)?,
                    })
                },
            mapper: |it| ProjectRow::from(it),
        }
    }
}
pub struct DeleteStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn delete() -> DeleteStmt {
    DeleteStmt("DELETE FROM projects WHERE id = $1", None)
}
impl DeleteStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub async fn bind<'c, 'a, 's, C: GenericClient>(
        &'s self,
        client: &'c C,
        id: &'a uuid::Uuid,
    ) -> Result<u64, tokio_postgres::Error> {
        client.execute(self.0, &[id]).await
    }
}
