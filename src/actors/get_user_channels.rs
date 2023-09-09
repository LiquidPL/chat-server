use anyhow::{anyhow, Error};
use tokio::sync::mpsc;

use crate::{
    database::Pool,
    models::{channel::ChannelUser, user::User},
    schema::channels,
    views::channel::ChannelDetails,
};

use super::ActorMessage;

use diesel::prelude::*;
use diesel_async::RunQueryDsl;

struct GetUserChannelsActor {
    db_pool: Pool,
    receiver: mpsc::Receiver<ActorMessage>,
}

pub struct GetUserChannelsActorHandle {
    pub sender: mpsc::Sender<ActorMessage>,
}

impl GetUserChannelsActorHandle {
    pub fn new(db_pool: Pool) -> Self {
        let (sender, receiver) = mpsc::channel::<ActorMessage>(32);

        let mut actor = GetUserChannelsActor { db_pool, receiver };

        tokio::spawn(async move { actor.run().await });

        Self { sender }
    }
}

impl GetUserChannelsActor {
    pub async fn run(&mut self) {
        while let Some(ActorMessage::GetUserChannels { user, respond_to }) =
            self.receiver.recv().await
        {
            let channels = self.get_channels(user).await;

            let _ = respond_to.send(channels);
        }
    }

    async fn get_channels(&mut self, user: User) -> Result<Vec<ChannelDetails>, Error> {
        let mut conn = self.db_pool.get().await?;

        ChannelUser::belonging_to(&user)
            .inner_join(channels::table)
            .select(ChannelDetails::as_select())
            .load(&mut conn)
            .await
            .map_err(|err| anyhow!(err.to_string()))
    }
}
