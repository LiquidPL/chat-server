use diesel_async::{pooled_connection::AsyncDieselConnectionManager, AsyncPgConnection};
use dotenvy::dotenv;
use std::env;

pub type Pool = bb8::Pool<AsyncDieselConnectionManager<AsyncPgConnection>>;

pub async fn get_connection_pool() -> Result<Pool, &'static str> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .map_err(|_| "DATABASE_URL must be set")?;

    let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(database_url);
    bb8::Pool::builder().build(config).await
        .map_err(|_| "Error while connecting to the database")
}
