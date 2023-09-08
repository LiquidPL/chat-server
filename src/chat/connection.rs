use std::ops::ControlFlow;

use crate::models::user::User;
use anyhow::{Error, anyhow};
use axum::extract::ws::{Message, WebSocket};
use futures::{
    stream::{SplitSink, SplitStream},
    StreamExt, SinkExt,
};
use tokio::sync::mpsc;

use super::server::Command;

struct Connection {
    receiver: SplitStream<WebSocket>,
    user: User,
    disconnect_sender: mpsc::Sender<Command>,
}

pub struct ConnectionHandle {
    sender: SplitSink<WebSocket, Message>,
}

impl Connection {
    async fn run(mut self) {
        while let Some(Ok(msg)) = self.receiver.next().await {
            if Self::process_message(msg).is_break() {
                let _ = self.disconnect_sender.send(Command::Disconnect(self.user)).await;
                break;
            }
        }
    }

    fn process_message(msg: Message) -> ControlFlow<(), ()> {
        match msg {
            Message::Text(t) => println!("{}", t),
            Message::Close(_) => {
                return ControlFlow::Break(());
            }
            _ => (),
        }
        ControlFlow::Continue(())
    }
}

impl ConnectionHandle {
    pub fn new(socket: WebSocket, user: User, disconnect_sender: mpsc::Sender<Command>) -> Self {
        let (sender, receiver) = socket.split();

        let connection = Connection {
            receiver,
            user,
            disconnect_sender,
        };

        tokio::spawn(async move { connection.run().await });

        Self { sender }
    }

    pub async fn send_message(&mut self, message: Message) -> Result<(), Error> {
        self.sender.send(message).await.map_err(|err| anyhow!(err.to_string()))
    }
}
