use diesel::prelude::*;

use serde::{Deserialize, Serialize};

use chrono::NaiveDateTime;

use super::channel::Channel;
use super::user::{User, UserId};
use crate::schema::messages;

#[derive(Clone, Queryable, Selectable, Identifiable, Associations, Serialize)]
#[diesel(table_name = messages)]
#[diesel(belongs_to(User, foreign_key = sender_id))]
#[diesel(belongs_to(Channel))]
pub struct Message {
    pub id: i32,
    pub sender_id: UserId,
    pub channel_id: i32,
    pub content: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = messages)]
pub struct NewMessage {
    pub content: String,
    #[serde(skip_deserializing)]
    pub sender_id: Option<UserId>,
    #[serde(skip_deserializing)]
    pub channel_id: Option<i32>,
}
