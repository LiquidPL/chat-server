use std::{env, sync::Arc};

use anyhow::anyhow;
use async_sqlx_session::PostgresSessionStore;
use axum::{extract::State, http::Request, middleware::Next, response::IntoResponse};
use axum_login::{axum_sessions::SessionLayer, AuthLayer, PostgresStore, SqlxStore};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use dotenvy::dotenv;
use hyper::{header, StatusCode};
use jsonwebtoken::{decode, DecodingKey, Validation};

use crate::{
    controllers::AppError,
    database::SqlxPool,
    models::user::{TokenClaims, User, UserId},
    state::AppState,
};

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

pub async fn jwt_auth<B>(
    State(state): State<Arc<AppState>>,
    mut auth: AuthContext,
    mut req: Request<B>,
    next: Next<B>,
) -> Result<impl IntoResponse, AppError> {
    let token = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .and_then(|value| {
            if value.starts_with("Bearer ") {
                Some(value[7..].to_owned())
            } else {
                None
            }
        })
        .ok_or_else(|| AppError {
            status_code: StatusCode::UNAUTHORIZED,
            error: anyhow!("unauthorized"),
        })?;

    let claims = decode::<TokenClaims>(
        &token,
        &DecodingKey::from_secret(state.config.secret.as_ref()),
        &Validation::default(),
    )
    .map_err(|_| AppError {
        status_code: StatusCode::UNAUTHORIZED,
        error: anyhow!("unauthorized"),
    })?
    .claims;

    let user_id = &claims.sub.parse::<UserId>()?;

    let mut conn = state.db_pool.get().await?;

    use crate::schema::users::dsl::*;

    let user = match users
        .filter(id.eq(user_id))
        .select(User::as_select())
        .first(&mut conn)
        .await
    {
        Ok(user) => Ok(user),
        Err(err) => match err {
            diesel::result::Error::NotFound => Err(AppError {
                status_code: StatusCode::UNAUTHORIZED,
                error: anyhow!("unauthorized"),
            }),
            err => Err(AppError::new(anyhow!(err))),
        },
    }?;

    auth.login(&user).await?;
    req.extensions_mut().insert(user);

    Ok(next.run(req).await)
}
