mod get_initial_sync;
mod validate_token;

use anyhow::Error;
use serde::Serialize;
use tokio::sync::oneshot;

use crate::views::message::MessageDetails;
use crate::{models::user::User, views::channel::ChannelDetails};

pub use self::get_initial_sync::GetInitialSyncActorHandle;
pub use self::validate_token::ValidateTokenActorHandle;

pub enum ActorMessage {
    ValidateToken {
        token: String,
        respond_to: oneshot::Sender<Result<User, Error>>,
    },
    GetInitialSync {
        user: User,
        respond_to: oneshot::Sender<Result<Vec<InitialChannelDetails>, Error>>,
    },
}

#[derive(Clone)]
pub struct UserAuthenticated {
    pub user: User,
    pub session_id: String,
}

/// Used to send initial data for the client to display upon connecting
/// (channel and the most recent message)
#[derive(Clone, Serialize)]
pub struct InitialChannelDetails {
    channel: ChannelDetails,
    message: Option<MessageDetails>,
}
