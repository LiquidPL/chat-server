mod get_user_channels;
mod validate_token;

use anyhow::Error;
use tokio::sync::oneshot;

use crate::{models::user::User, views::channel::ChannelDetails};

pub use self::get_user_channels::GetUserChannelsActorHandle;
pub use self::validate_token::ValidateTokenActorHandle;

pub enum ActorMessage {
    ValidateToken {
        token: String,
        respond_to: oneshot::Sender<Result<User, Error>>,
    },
    GetUserChannels {
        user: User,
        respond_to: oneshot::Sender<Result<Vec<ChannelDetails>, Error>>,
    },
}

#[derive(Clone)]
pub struct UserAuthenticated {
    pub user: User,
    pub session_id: String,
}
