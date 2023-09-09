use anyhow::{anyhow, Error};
use tokio::sync::mpsc;

use crate::{
    database::Pool,
    models::{channel::ChannelUser, user::User},
    views::{channel::ChannelDetails, message::MessageDetails},
};

use super::{ActorMessage, InitialChannelDetails};

use diesel::prelude::*;
use diesel_async::RunQueryDsl;

struct GetInitialSyncActor {
    db_pool: Pool,
    receiver: mpsc::Receiver<ActorMessage>,
}

/// Retrieves the "initial sync" data for a given user, ie. the state necessary
/// for a client to render the initial application and start receiving messages
/// from the server.
///
/// The initial sync payload consists of a list of all channels this user
/// belongs to, alongside the most recent message for each channel.
pub struct GetInitialSyncActorHandle {
    pub sender: mpsc::Sender<ActorMessage>,
}

impl GetInitialSyncActorHandle {
    pub fn new(db_pool: Pool) -> Self {
        let (sender, receiver) = mpsc::channel::<ActorMessage>(32);

        let mut actor = GetInitialSyncActor { db_pool, receiver };

        tokio::spawn(async move { actor.run().await });

        Self { sender }
    }
}

impl GetInitialSyncActor {
    pub async fn run(&mut self) {
        while let Some(ActorMessage::GetInitialSync { user, respond_to }) =
            self.receiver.recv().await
        {
            let channels = self.prepare_initial_sync_payload(user).await;

            let _ = respond_to.send(channels);
        }
    }

    async fn prepare_initial_sync_payload(&mut self, user: User) -> Result<Vec<InitialChannelDetails>, Error> {
        let channels = self.get_channels(user).await?;
        let messages = self.get_most_recent_messages(&channels).await?;


        let mut initial_channels: Vec<InitialChannelDetails> = Vec::new();

        for channel in channels {
            let message = messages.iter().find(|&message| message.channel_id == channel.id);

            initial_channels.push(InitialChannelDetails {
                channel,
                message: message.map_or(None, |message| Some(message.clone())),
            });
        }

        Ok(initial_channels)
    }

    async fn get_channels(&mut self, user: User) -> Result<Vec<ChannelDetails>, Error> {
        use crate::schema::channels::dsl::*;

        let mut conn = self.db_pool.get().await?;

        ChannelUser::belonging_to(&user)
            .inner_join(channels)
            .order(id.desc())
            .select(ChannelDetails::as_select())
            .load(&mut conn)
            .await
            .map_err(|err| anyhow!(err.to_string()))
    }

    async fn get_most_recent_messages(&mut self, channels: &Vec<ChannelDetails>) -> Result<Vec<MessageDetails>, Error> {
        use crate::schema::messages::dsl::*;

        let channel_ids: Vec<i32> = channels.iter().map(|channel| channel.id).collect();

        let mut conn = self.db_pool.get().await?;

        messages
            .filter(channel_id.eq_any(channel_ids))
            .distinct_on(channel_id)
            .order((channel_id.asc(), created_at.desc()))
            .select(MessageDetails::as_select())
            .load(&mut conn)
            .await
            .map_err(|err| anyhow!(err.to_string()))
    }
}
