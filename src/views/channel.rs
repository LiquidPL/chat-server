use chrono::NaiveDateTime;
use diesel::prelude::Queryable;
use diesel::Selectable;
use serde::Serialize;

use crate::models::channel::Channel;
use crate::models::user::UserId;
use crate::schema::channels;

use super::user::UserDetails;

#[derive(Clone, Serialize, Selectable, Queryable)]
#[diesel(table_name = channels)]
pub struct ChannelDetails {
    pub id: i32,
    pub name: String,
    pub owner_id: UserId,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Clone, Serialize)]
pub struct ChannelDetailsWithUser {
    #[serde(flatten)]
    pub details: ChannelDetails,
    pub members: Vec<UserDetails>,
}

impl From<Channel> for ChannelDetails {
    fn from(value: Channel) -> Self {
        Self {
            id: value.id,
            name: value.name,
            owner_id: value.owner_id,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

impl ChannelDetailsWithUser {
    pub fn new(details: ChannelDetails, members: Vec<UserDetails>) -> Self {
        Self { details, members }
    }
}
