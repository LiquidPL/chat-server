use std::collections::HashMap;

use anyhow::{Error, anyhow};
use axum::extract::ws::{Message, WebSocket};
use tokio::sync::mpsc;

use crate::{
    models::user::{User, UserId},
    views::chat::UserStatus,
};

use super::connection::ConnectionHandle;

pub enum Command {
    Send { destination: User, message: String },
    Connect { user: User, socket: WebSocket },
    Disconnect(User),
}

struct ChatServer {
    sender: mpsc::Sender<Command>,
    receiver: mpsc::Receiver<Command>,
    connections: HashMap<UserId, ConnectionHandle>,
    user_statuses: HashMap<UserId, UserStatus>,
}

pub struct ChatServerHandle {
    sender: mpsc::Sender<Command>,
}

impl ChatServerHandle {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel::<Command>(32);

        let chat_server = ChatServer::new(sender.clone(), receiver);

        tokio::spawn(async move { chat_server.run().await });

        Self { sender }
    }

    pub async fn send_command(&self, command: Command) -> Result<(), Error> {
        self.sender.send(command).await.map_err(|err| anyhow!(err.to_string()))
    }
}

impl ChatServer {
    pub fn new(sender: mpsc::Sender<Command>, receiver: mpsc::Receiver<Command>) -> Self {
        ChatServer { sender, receiver, connections: HashMap::new(), user_statuses: HashMap::new() }
    }

    pub async fn run(mut self) {
        while let Some(cmd) = self.receiver.recv().await {
            use Command::*;

            match cmd {
                Send {
                    destination,
                    message,
                } => {
                    if let Some(conn) = self.connections.get_mut(&destination.id) {
                        conn.send_message(Message::Text(message)).await.unwrap();
                    }
                }
                Connect { user, socket } => {
                    let user_id = user.id;
                    let connection = ConnectionHandle::new(socket, user, self.sender.clone());

                    self.connections.insert(user_id, connection);
                    self.user_statuses.insert(user_id, UserStatus::Online);

                    println!("User id={} connected", user_id);
                }
                Disconnect(user) => {
                    self.connections.remove(&user.id).unwrap();
                    self.user_statuses.remove(&user.id).unwrap();
                    println!("User id={} disconnected", user.id);
                }
            }
        }
    }
    // async fn send_ready_message(self, user: &User) {
    //     let tx = self.get_manager_tx();
    // }
}
