use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::{
    models::user::UserId,
    views::{channel::ChannelDetails, message::MessageDetails, user::UserDetails}, actors::InitialChannelDetails,
};

#[derive(Clone, Serialize)]
#[serde(tag = "event_type", content = "data")]
pub enum ServerEvent {
    UserAuthenticated {
        user: UserDetails,
        channels: Vec<InitialChannelDetails>,
    },
    AuthenticationError {
        error: String,
    },
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
    MessageCreated {
        id: i32,
        sender_id: UserId,
        channel_id: i32,
        content: String,
        created_at: NaiveDateTime,
    },
    MessageDeleted {
        id: i32,
    },
}

#[derive(Deserialize)]
#[serde(tag = "event_type", content = "data")]
pub enum ClientEvent {
    Auth { token: String },
}

impl ServerEvent {
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

    pub fn message_created(message: &MessageDetails) -> Self {
        Self::MessageCreated {
            id: message.id,
            sender_id: message.sender_id,
            channel_id: message.channel_id,
            content: message.content.clone(),
            created_at: message.created_at,
        }
    }

    pub fn message_deleted(message: &MessageDetails) -> Self {
        Self::MessageDeleted { id: message.id }
    }
}
