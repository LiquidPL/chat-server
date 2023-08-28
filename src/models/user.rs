use axum_login::{secrecy::SecretVec, AuthUser};
use diesel::prelude::*;
use serde::Deserialize;

#[derive(Clone, Queryable, Selectable, sqlx::FromRow)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
}

impl AuthUser<i32> for User {
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
