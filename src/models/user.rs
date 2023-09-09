use axum_login::{secrecy::SecretVec, AuthUser};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

pub type UserId = i32;

#[derive(Clone, Queryable, Selectable, Identifiable, sqlx::FromRow)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: UserId,
    pub username: String,
    pub password: String,
}

impl AuthUser<UserId> for User {
    fn get_id(&self) -> i32 {
        self.id
    }

    fn get_password_hash(&self) -> SecretVec<u8> {
        SecretVec::new(self.password.clone().into())
    }
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = crate::schema::users)]
pub struct UserRegistration {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct UserAuthentication {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Serialize)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}
