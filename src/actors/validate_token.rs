use anyhow::{anyhow, Error};
use tokio::sync::mpsc;

use diesel::prelude::*;
use diesel_async::RunQueryDsl;

use crate::{
    auth::decode_access_token,
    config::Config,
    database::Pool,
    models::user::{User, UserId},
};

use super::ActorMessage;

struct ValidateTokenActor {
    db_pool: Pool,
    config: Config,
    receiver: mpsc::Receiver<ActorMessage>,
}

pub struct ValidateTokenActorHandle {
    pub sender: mpsc::Sender<ActorMessage>,
}

impl ValidateTokenActorHandle {
    pub fn new(db_pool: Pool, config: Config) -> Self {
        let (sender, receiver) = mpsc::channel::<ActorMessage>(32);

        let mut actor = ValidateTokenActor {
            db_pool,
            config,
            receiver,
        };

        tokio::spawn(async move { actor.run().await });

        Self { sender }
    }
}

impl ValidateTokenActor {
    pub async fn run(&mut self) {
        while let Some(ActorMessage::ValidateToken { token, respond_to }) =
            self.receiver.recv().await
        {
            let validated = self
                .validate_token(&token)
                .await;

            let _ = respond_to.send(validated);
        }
    }

    async fn validate_token(&mut self, token: &String) -> Result<User, Error> {
        let claims = decode_access_token(&token, self.config.secret.as_ref())?;
        let user_id = &claims.sub.parse::<UserId>()?;

        let mut conn = self.db_pool.get().await?;

        use crate::schema::users::dsl::*;

        match users
            .filter(id.eq(user_id))
            .select(User::as_select())
            .first(&mut conn)
            .await
        {
            Ok(user) => Ok(user),
            Err(error) => match error {
                diesel::result::Error::NotFound => Err(anyhow!("unauthorized")),
                error => Err(anyhow!(error)),
            },
        }
    }
}
