use chrono::NaiveDateTime;
use diesel::prelude::Queryable;
use diesel::Selectable;
use serde::Serialize;

use crate::models::user::UserId;
use crate::schema::channels;

#[derive(Serialize, Selectable, Queryable)]
#[diesel(table_name = channels)]
pub struct ChannelDetails {
    pub id: i32,
    pub name: String,
    pub owner_id: UserId,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
