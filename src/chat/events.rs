use chrono::NaiveDateTime;
use serde::Serialize;

use crate::{models::user::UserId, views::channel::ChannelDetails};

#[derive(Serialize)]
#[serde(tag = "event_type", content = "data")]
pub enum Event {
    ChannelCreated {
        id: i32,
        name: String,
        owner_id: UserId,
        created_at: NaiveDateTime,
    },
    ChannelDeleted {
        id: i32,
        name: String,
    },
}

impl Event {
    pub fn channel_created(channel: &ChannelDetails) -> Self {
        Self::ChannelCreated {
            id: channel.id,
            name: channel.name.clone(),
            owner_id: channel.owner_id,
            created_at: channel.created_at,
        }
    }

    pub fn channel_deleted(channel: &ChannelDetails) -> Self {
        Self::ChannelDeleted {
            id: channel.id,
            name: channel.name.clone(),
        }
    }
}
