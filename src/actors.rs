mod validate_token;

use anyhow::Error;
use tokio::sync::oneshot;

use crate::models::user::User;

pub use self::validate_token::ValidateTokenActorHandle;

pub enum ActorMessage {
    ValidateToken {
        token: String,
        respond_to: oneshot::Sender<Result<User, Error>>,
    },
}

#[derive(Clone)]
pub struct UserAuthenticated {
    pub user: User,
    pub session_id: String,
}
