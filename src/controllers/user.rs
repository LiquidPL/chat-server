use axum::{
    extract::State,
    Json,
};

use crate::{models::user::NewUser, views::user::User, server::AppState};

use diesel::SelectableHelper;
use diesel_async::RunQueryDsl;

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};

use super::AppError;

pub async fn create_user(
    State(state): State<AppState>,
    Json(mut new_user): Json<NewUser>,
) -> Result<Json<User>, AppError> {
    use crate::schema::users::dsl::*;

    let mut conn = state.db_pool.get().await?;

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    new_user.password = argon2
        .hash_password(new_user.password.as_bytes(), &salt)?
        .to_string();

    let res = diesel::insert_into(users)
        .values(new_user)
        .returning(User::as_returning())
        .get_result(&mut conn)
        .await?;

    Ok(Json(res))
}
