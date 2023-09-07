use crate::{models::user::User, schema::users};
use diesel::{Queryable, Selectable};
use serde::Serialize;

#[derive(Serialize, Selectable, Queryable)]
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

#[derive(Serialize)]
pub struct UserLogin {
    pub user: UserDetails,
    pub token: String,
}
