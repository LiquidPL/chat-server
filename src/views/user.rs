use crate::{models::user::User, schema::users};
use diesel::{Queryable, Selectable};
use serde::Serialize;

#[derive(Clone, Serialize, Selectable, Queryable)]
#[diesel(table_name = users)]
pub struct UserDetails {
    pub id: i32,
    pub username: String,
}

impl From<User> for UserDetails {
    fn from(user: User) -> Self {
        UserDetails {
            id: user.id,
            username: user.username,
        }
    }
}

impl From<UserDetails> for User {
    fn from(user: UserDetails) -> Self {
        Self {
            id: user.id,
            username: user.username,
            password: String::from(""),
        }
    }
}

#[derive(Serialize)]
pub struct UserLogin {
    pub user: UserDetails,
    pub token: String,
}
