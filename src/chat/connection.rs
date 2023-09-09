use std::ops::ControlFlow;

use crate::{
    actors::{UserAuthenticated, ValidateTokenActorHandle},
    models::user::User,
};
use anyhow::{anyhow, Error};
use axum::extract::ws::{Message, WebSocket};
use futures::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use rand::{distributions, Rng};
use tokio::{
    sync::{mpsc, oneshot},
    task::JoinHandle,
};

use super::{
    events::{ClientEvent, ServerEvent},
    server::Command,
};

struct Connection {
    session_id: String,
    receiver: SplitStream<WebSocket>,
    validate_token: ValidateTokenActorHandle,
    auth_sender: mpsc::Sender<Result<UserAuthenticated, Error>>,
    disconnect_sender: mpsc::Sender<Command>,
}

pub struct ConnectionHandle {
    sender: SplitSink<WebSocket, Message>,
    join_handle: JoinHandle<()>,
}

impl Connection {
    async fn run(mut self) {
        while let Some(Ok(msg)) = self.receiver.next().await {
            if self.handle_websocket_message(msg).await.is_break() {
                let _ = self
                    .disconnect_sender
                    .send(Command::Disconnect {
                        session_id: self.session_id,
                    })
                    .await;
                break;
            }
        }
    }

    async fn handle_websocket_message(&self, msg: Message) -> ControlFlow<(), ()> {
        match msg {
            Message::Text(raw_text) => {
                let event: Result<ClientEvent, serde_json::Error> = serde_json::from_str(&raw_text);

                if let Ok(event) = event {
                    match event {
                        ClientEvent::Auth { token } => {
                            let (sender, receiver) = oneshot::channel::<Result<User, Error>>();

                            let _ = self
                                .validate_token
                                .sender
                                .send(crate::actors::ActorMessage::ValidateToken {
                                    token,
                                    respond_to: sender,
                                })
                                .await;

                            let user = receiver
                                .await
                                .map_err(|err| anyhow!(err.to_string()))
                                .and_then(|val| val.map_err(|err| anyhow!(err.to_string())));

                            if let Ok(user) = user {
                                let _ = self
                                    .auth_sender
                                    .send(Ok(UserAuthenticated {
                                        user: user.clone(),
                                        session_id: self.session_id.clone(),
                                    }))
                                    .await;
                            }
                        }
                    }
                }
            }
            Message::Close(_) => {
                return ControlFlow::Break(());
            }
            _ => (),
        }
        ControlFlow::Continue(())
    }
}

impl ConnectionHandle {
    pub fn new(
        socket: WebSocket,
        validate_token: ValidateTokenActorHandle,
        auth_sender: mpsc::Sender<Result<UserAuthenticated, Error>>,
        disconnect_sender: mpsc::Sender<Command>,
    ) -> Self {
        let (sender, receiver) = socket.split();

        let connection = Connection {
            session_id: rand::thread_rng()
                .sample_iter(distributions::Alphanumeric)
                .take(24)
                .map(char::from)
                .collect(),
            receiver,
            validate_token,
            auth_sender,
            disconnect_sender,
        };

        let join_handle = tokio::spawn(async move { connection.run().await });

        Self {
            sender,
            join_handle,
        }
    }

    pub async fn send_message(&mut self, message: ServerEvent) -> Result<(), Error> {
        self.sender
            .send(Message::Text(serde_json::to_string(&message).unwrap()))
            .await
            .map_err(|err| anyhow!(err.to_string()))
    }

    pub async fn close(&mut self) {
        self.join_handle.abort();
        let _ = self.sender.close().await;
    }
}
