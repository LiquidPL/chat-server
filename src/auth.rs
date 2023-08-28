use std::env;

use anyhow::anyhow;
use async_sqlx_session::PostgresSessionStore;
use axum_login::{axum_sessions::SessionLayer, AuthLayer, PostgresStore, SqlxStore};
use dotenvy::dotenv;

use crate::{database::SqlxPool, models::user::User};

pub type AuthContext = axum_login::extractors::AuthContext<i32, User, SqlxStore<SqlxPool, User>>;

pub async fn create_auth(
    pool: SqlxPool,
) -> Result<
    (
        SessionLayer<PostgresSessionStore>,
        AuthLayer<SqlxStore<SqlxPool, User>, i32, User>,
    ),
    anyhow::Error,
> {
    dotenv().ok();

    let secret_env = env::var("SECRET").map_err(|_| anyhow!("SECRET env var must be set"))?;

    let secret = secret_env.as_bytes();

    let session_store = PostgresSessionStore::from_client(pool.clone());
    session_store.migrate().await?;

    let session_layer = SessionLayer::new(session_store, &secret);

    let user_store = PostgresStore::<User>::new(pool.clone());
    let auth_layer = AuthLayer::new(user_store, &secret);

    Ok((session_layer, auth_layer))
}
