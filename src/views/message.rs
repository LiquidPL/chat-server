use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::Serialize;

use crate::models::user::UserId;
use crate::schema::messages;

#[derive(Clone, Serialize, Selectable, Queryable)]
#[diesel(table_name = messages)]
pub struct MessageDetails {
    pub id: i32,
    pub sender_id: UserId,
    pub channel_id: i32,
    pub content: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
