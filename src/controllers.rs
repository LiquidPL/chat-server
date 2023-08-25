use axum::http::StatusCode;

pub mod user {
    use axum::{extract::State, Json, http::StatusCode};

    use crate::{database::Pool, views::user::User, models::NewUser, schema::users};

    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;

    use argon2::{
        password_hash::{
            rand_core::OsRng,
            PasswordHasher, SaltString,
        },
        Argon2
    };

    use super::internal_error;

    pub async fn create_user(
        State(pool): State<Pool>,
        Json(mut new_user): Json<NewUser>,
    ) -> Result<Json<User>, (StatusCode, String)> {
        let mut conn = pool.get().await
            .map_err(internal_error)?;

        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        new_user.password = argon2.hash_password(new_user.password.as_bytes(), &salt)
            .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Error while creating account".to_string()))?.to_string();

        let res = diesel::insert_into(users::table)
            .values(new_user)
            .returning(User::as_returning())
            .get_result(&mut conn)
            .await
            .map_err(internal_error)?;

        Ok(Json(res))
    }
}

fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
