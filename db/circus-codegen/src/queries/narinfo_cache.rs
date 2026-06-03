// This file was generated with `clorinde`. Do not modify.

#[derive(Debug)]
pub struct UpsertParams<
    T1: crate::StringSql,
    T2: crate::StringSql,
    T3: crate::StringSql,
    T4: crate::StringSql,
    T5: crate::StringSql,
    T6: crate::StringSql,
    T7: crate::StringSql,
    T8: crate::ArraySql<Item = T7>,
    T9: crate::StringSql,
    T10: crate::StringSql,
> {
    pub store_path: T1,
    pub nar_hash: T2,
    pub nar_size: i64,
    pub file_hash: Option<T3>,
    pub file_size: Option<i64>,
    pub compression: T4,
    pub url: T5,
    pub deriver: Option<T6>,
    pub references: T8,
    pub sig: Option<T9>,
    pub ca: Option<T10>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct NarinfoCacheRow {
    pub store_path: String,
    pub nar_hash: String,
    pub nar_size: i64,
    pub file_hash: Option<String>,
    pub file_size: Option<i64>,
    pub compression: String,
    pub url: String,
    pub deriver: Option<String>,
    pub references: Vec<String>,
    pub sig: Option<String>,
    pub ca: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
pub struct NarinfoCacheRowBorrowed<'a> {
    pub store_path: &'a str,
    pub nar_hash: &'a str,
    pub nar_size: i64,
    pub file_hash: Option<&'a str>,
    pub file_size: Option<i64>,
    pub compression: &'a str,
    pub url: &'a str,
    pub deriver: Option<&'a str>,
    pub references: crate::ArrayIterator<'a, &'a str>,
    pub sig: Option<&'a str>,
    pub ca: Option<&'a str>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
impl<'a> From<NarinfoCacheRowBorrowed<'a>> for NarinfoCacheRow {
    fn from(
        NarinfoCacheRowBorrowed {
            store_path,
            nar_hash,
            nar_size,
            file_hash,
            file_size,
            compression,
            url,
            deriver,
            references,
            sig,
            ca,
            created_at,
            updated_at,
        }: NarinfoCacheRowBorrowed<'a>,
    ) -> Self {
        Self {
            store_path: store_path.into(),
            nar_hash: nar_hash.into(),
            nar_size,
            file_hash: file_hash.map(|v| v.into()),
            file_size,
            compression: compression.into(),
            url: url.into(),
            deriver: deriver.map(|v| v.into()),
            references: references.map(|v| v.into()).collect(),
            sig: sig.map(|v| v.into()),
            ca: ca.map(|v| v.into()),
            created_at,
            updated_at,
        }
    }
}
use crate::client::async_::GenericClient;
use futures::{self, StreamExt, TryStreamExt};
pub struct NarinfoCacheRowQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<NarinfoCacheRowBorrowed, tokio_postgres::Error>,
    mapper: fn(NarinfoCacheRowBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> NarinfoCacheRowQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(NarinfoCacheRowBorrowed) -> R,
    ) -> NarinfoCacheRowQuery<'c, 'a, 's, C, R, N> {
        NarinfoCacheRowQuery {
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
pub struct UpsertStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn upsert() -> UpsertStmt {
    UpsertStmt(
        "INSERT INTO narinfo_cache ( store_path, nar_hash, nar_size, file_hash, file_size, compression, url, deriver, \"references\", sig, ca, updated_at ) VALUES ( $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, NOW() ) ON CONFLICT (store_path) DO UPDATE SET nar_hash = EXCLUDED.nar_hash, nar_size = EXCLUDED.nar_size, file_hash = EXCLUDED.file_hash, file_size = EXCLUDED.file_size, compression = EXCLUDED.compression, url = EXCLUDED.url, deriver = EXCLUDED.deriver, \"references\" = EXCLUDED.\"references\", sig = EXCLUDED.sig, ca = EXCLUDED.ca, updated_at = NOW()",
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
    pub async fn bind<
        'c,
        'a,
        's,
        C: GenericClient,
        T1: crate::StringSql,
        T2: crate::StringSql,
        T3: crate::StringSql,
        T4: crate::StringSql,
        T5: crate::StringSql,
        T6: crate::StringSql,
        T7: crate::StringSql,
        T8: crate::ArraySql<Item = T7>,
        T9: crate::StringSql,
        T10: crate::StringSql,
    >(
        &'s self,
        client: &'c C,
        store_path: &'a T1,
        nar_hash: &'a T2,
        nar_size: &'a i64,
        file_hash: &'a Option<T3>,
        file_size: &'a Option<i64>,
        compression: &'a T4,
        url: &'a T5,
        deriver: &'a Option<T6>,
        references: &'a T8,
        sig: &'a Option<T9>,
        ca: &'a Option<T10>,
    ) -> Result<u64, tokio_postgres::Error> {
        client
            .execute(
                self.0,
                &[
                    store_path,
                    nar_hash,
                    nar_size,
                    file_hash,
                    file_size,
                    compression,
                    url,
                    deriver,
                    references,
                    sig,
                    ca,
                ],
            )
            .await
    }
}
impl<
    'a,
    C: GenericClient + Send + Sync,
    T1: crate::StringSql,
    T2: crate::StringSql,
    T3: crate::StringSql,
    T4: crate::StringSql,
    T5: crate::StringSql,
    T6: crate::StringSql,
    T7: crate::StringSql,
    T8: crate::ArraySql<Item = T7>,
    T9: crate::StringSql,
    T10: crate::StringSql,
>
    crate::client::async_::Params<
        'a,
        'a,
        'a,
        UpsertParams<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10>,
        std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        >,
        C,
    > for UpsertStmt
{
    fn params(
        &'a self,
        client: &'a C,
        params: &'a UpsertParams<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10>,
    ) -> std::pin::Pin<
        Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
    > {
        Box::pin(self.bind(
            client,
            &params.store_path,
            &params.nar_hash,
            &params.nar_size,
            &params.file_hash,
            &params.file_size,
            &params.compression,
            &params.url,
            &params.deriver,
            &params.references,
            &params.sig,
            &params.ca,
        ))
    }
}
pub struct GetStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn get() -> GetStmt {
    GetStmt("SELECT * FROM narinfo_cache WHERE store_path =$1", None)
}
impl GetStmt {
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
        store_path: &'a T1,
    ) -> NarinfoCacheRowQuery<'c, 'a, 's, C, NarinfoCacheRow, 1> {
        NarinfoCacheRowQuery {
            client,
            params: [store_path],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |
                row: &tokio_postgres::Row,
            | -> Result<NarinfoCacheRowBorrowed, tokio_postgres::Error> {
                Ok(NarinfoCacheRowBorrowed {
                    store_path: row.try_get(0)?,
                    nar_hash: row.try_get(1)?,
                    nar_size: row.try_get(2)?,
                    file_hash: row.try_get(3)?,
                    file_size: row.try_get(4)?,
                    compression: row.try_get(5)?,
                    url: row.try_get(6)?,
                    deriver: row.try_get(7)?,
                    references: row.try_get(8)?,
                    sig: row.try_get(9)?,
                    ca: row.try_get(10)?,
                    created_at: row.try_get(11)?,
                    updated_at: row.try_get(12)?,
                })
            },
            mapper: |it| NarinfoCacheRow::from(it),
        }
    }
}
pub struct GetByHashPartStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn get_by_hash_part() -> GetByHashPartStmt {
    GetByHashPartStmt("SELECT * FROM narinfo_cache WHERE store_path LIKE $1", None)
}
impl GetByHashPartStmt {
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
        hash_part_pattern: &'a T1,
    ) -> NarinfoCacheRowQuery<'c, 'a, 's, C, NarinfoCacheRow, 1> {
        NarinfoCacheRowQuery {
            client,
            params: [hash_part_pattern],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |
                row: &tokio_postgres::Row,
            | -> Result<NarinfoCacheRowBorrowed, tokio_postgres::Error> {
                Ok(NarinfoCacheRowBorrowed {
                    store_path: row.try_get(0)?,
                    nar_hash: row.try_get(1)?,
                    nar_size: row.try_get(2)?,
                    file_hash: row.try_get(3)?,
                    file_size: row.try_get(4)?,
                    compression: row.try_get(5)?,
                    url: row.try_get(6)?,
                    deriver: row.try_get(7)?,
                    references: row.try_get(8)?,
                    sig: row.try_get(9)?,
                    ca: row.try_get(10)?,
                    created_at: row.try_get(11)?,
                    updated_at: row.try_get(12)?,
                })
            },
            mapper: |it| NarinfoCacheRow::from(it),
        }
    }
}
pub struct GetByUrlStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn get_by_url() -> GetByUrlStmt {
    GetByUrlStmt(
        "SELECT * FROM narinfo_cache WHERE url =$1 ORDER BY updated_at DESC LIMIT 1",
        None,
    )
}
impl GetByUrlStmt {
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
        url: &'a T1,
    ) -> NarinfoCacheRowQuery<'c, 'a, 's, C, NarinfoCacheRow, 1> {
        NarinfoCacheRowQuery {
            client,
            params: [url],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |
                row: &tokio_postgres::Row,
            | -> Result<NarinfoCacheRowBorrowed, tokio_postgres::Error> {
                Ok(NarinfoCacheRowBorrowed {
                    store_path: row.try_get(0)?,
                    nar_hash: row.try_get(1)?,
                    nar_size: row.try_get(2)?,
                    file_hash: row.try_get(3)?,
                    file_size: row.try_get(4)?,
                    compression: row.try_get(5)?,
                    url: row.try_get(6)?,
                    deriver: row.try_get(7)?,
                    references: row.try_get(8)?,
                    sig: row.try_get(9)?,
                    ca: row.try_get(10)?,
                    created_at: row.try_get(11)?,
                    updated_at: row.try_get(12)?,
                })
            },
            mapper: |it| NarinfoCacheRow::from(it),
        }
    }
}
pub struct CountStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn count() -> CountStmt {
    CountStmt("SELECT COUNT(*) FROM narinfo_cache", None)
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
