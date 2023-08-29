use chrono::NaiveDateTime;
use diesel::prelude::*;

use serde::{Deserialize, Serialize};

use super::user::{User, UserId};
use crate::schema::{channels, channels_users};

#[derive(Clone, Queryable, Selectable, Identifiable, Associations, Serialize)]
#[diesel(belongs_to(User, foreign_key = owner_id))]
#[diesel(table_name = channels)]
pub struct Channel {
    pub id: i32,
    pub name: String,
    pub owner_id: UserId,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Identifiable, Selectable, Queryable, Associations)]
#[diesel(belongs_to(Channel))]
#[diesel(belongs_to(User))]
#[diesel(table_name = channels_users)]
#[diesel(primary_key(channel_id, user_id))]
pub struct ChannelUser {
    pub channel_id: i32,
    pub user_id: UserId,
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = channels)]
pub struct NewChannel {
    pub name: String,
    #[serde(skip_deserializing)]
    pub owner_id: Option<i32>,
}
