use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::{request::Parts, StatusCode},
};
use sqlx::Sqlite;
use sqlx::SqlitePool;
use sqlx::{migrate::MigrateError, pool::PoolConnection};
use sqlx::{sqlite::SqliteConnectOptions, Error};

pub struct DatabaseConnection(pub PoolConnection<Sqlite>);

#[async_trait]
impl<S> FromRequestParts<S> for DatabaseConnection
where
    SqlitePool: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let pool = SqlitePool::from_ref(state);

        let conn = pool
            .acquire()
            .await
            .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;

        Ok(Self(conn))
    }
}

pub async fn database_pool() -> anyhow::Result<SqlitePool> {
    let options = SqliteConnectOptions::new()
        .filename("blockchain.db")
        .create_if_missing(true);
    SqlitePool::connect_with(options).await.map_err(Error::into)
}

pub async fn run_migration(pool: &SqlitePool) -> Result<(), MigrateError> {
    sqlx::migrate!().run(pool).await
}
