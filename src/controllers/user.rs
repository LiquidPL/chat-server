use std::sync::Arc;

use anyhow::anyhow;
use axum::{
    extract::{Query, State},
    Json,
};
use hyper::StatusCode;

use crate::{
    auth::AuthContext,
    models::user::{User, UserAuthentication, UserRegistration},
    state::AppState,
    views::user::UserDetails,
};

use diesel::{SelectableHelper, QueryDsl, ExpressionMethods};
use diesel_async::RunQueryDsl;

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2, PasswordHash, PasswordVerifier,
};

use super::AppError;

pub async fn create_user(
    State(state): State<Arc<AppState>>,
    Json(mut user_registration): Json<UserRegistration>,
) -> Result<Json<UserDetails>, AppError> {
    use crate::schema::users::dsl::*;

    let mut conn = state.db_pool.get().await?;

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    user_registration.password = argon2
        .hash_password(user_registration.password.as_bytes(), &salt)?
        .to_string();

    let res = diesel::insert_into(users)
        .values(user_registration)
        .returning(UserDetails::as_returning())
        .get_result(&mut conn)
        .await?;

    Ok(Json(res))
}

pub async fn login(
    State(state): State<Arc<AppState>>,
    Query(params): Query<UserAuthentication>,
    mut auth: AuthContext,
) -> Result<Json<UserDetails>, AppError> {
    let mut conn = state.sqlx_db_pool.acquire().await?;

    let user: User = sqlx::query_as("select * from users where username = $1")
        .bind(params.username)
        .fetch_one(&mut conn)
        .await?;

    let parsed_hash = PasswordHash::new(&user.password)?;
    match Argon2::default().verify_password(params.password.as_bytes(), &parsed_hash) {
        Ok(_) => {
            auth.login(&user).await?;
            Ok(Json(user.into()))
        }
        Err(err) => match err {
            argon2::password_hash::Error::Password => Err(AppError {
                status_code: StatusCode::FORBIDDEN,
                error: anyhow!(err),
            }),
            other_error => Err(AppError::new(anyhow!(other_error))),
        },
    }
}

pub async fn logout(mut auth: AuthContext) {
    auth.logout().await;
}

pub async fn current_user(
    State(state): State<Arc<AppState>>,
    auth: AuthContext,
) -> Result<Json<UserDetails>, AppError> {
    use crate::schema::users::dsl::*;

    let mut conn = state.db_pool.get().await?;

    let user = users
        .filter(id.eq(auth.current_user.expect("route is auth guarded").id))
        .select(UserDetails::as_select())
        .first(&mut conn)
        .await?;

    Ok(Json(user))
}
