use crate::schema::users;
use diesel::{Selectable, Queryable};

#[derive(serde::Serialize, Selectable, Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
}
