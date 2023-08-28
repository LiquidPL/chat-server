use dotenvy::dotenv;
use std::env;

use diesel_async::{pooled_connection::AsyncDieselConnectionManager, AsyncPgConnection};
use sqlx::postgres::{PgPoolOptions, Postgres};
use sqlx::Pool as SqlxPoolBase;

pub type Pool = bb8::Pool<AsyncDieselConnectionManager<AsyncPgConnection>>;
pub type SqlxPool = SqlxPoolBase<Postgres>;

pub async fn create_connection_pool() -> Result<Pool, &'static str> {
    dotenv().ok();

    let database_url =
        env::var("DATABASE_URL").map_err(|_| "DATABASE_URL env variable must be set")?;

    let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(database_url);
    bb8::Pool::builder()
        .build(config)
        .await
        .map_err(|_| "Error while connecting to the database")
}

pub async fn create_sqlx_pool() -> Result<SqlxPoolBase<Postgres>, &'static str> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").map_err(|_| "DATABASE_URL must be set")?;

    PgPoolOptions::new()
        .connect(&database_url)
        .await
        .map_err(|_| "Error while connecting to the database")
}
