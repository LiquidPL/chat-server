use std::{
    collections::{HashMap, HashSet},
    time::Duration,
};

use anyhow::{anyhow, Error};
use axum::extract::ws::WebSocket;
use tokio::sync::mpsc;

use crate::{
    actors::{UserAuthenticated, ValidateTokenActorHandle},
    chat::events::ServerEvent,
    config::Config,
    database::Pool,
    models::user::{User, UserId},
    views::chat::UserStatus,
};

use super::connection::ConnectionHandle;

pub enum Command {
    Send {
        destination: User,
        message: ServerEvent,
    },
    Connect {
        socket: WebSocket,
    },
    Disconnect {
        session_id: String,
    },
}

struct ConnectionState {
    connection: ConnectionHandle,
    user: User,
}

struct ChatServer {
    db_pool: Pool,
    config: Config,
    sender: mpsc::Sender<Command>,
    receiver: mpsc::Receiver<Command>,
    connections: HashMap<String, ConnectionState>,
    active_sessions: HashMap<UserId, HashSet<String>>,
    user_statuses: HashMap<UserId, UserStatus>,
}

pub struct ChatServerHandle {
    sender: mpsc::Sender<Command>,
}

impl ChatServerHandle {
    pub fn new(db_pool: Pool, config: Config) -> Self {
        let (sender, receiver) = mpsc::channel::<Command>(32);

        let chat_server = ChatServer::new(db_pool, config, sender.clone(), receiver);

        tokio::spawn(async move { chat_server.run().await });

        Self { sender }
    }

    pub async fn send_command(&self, command: Command) -> Result<(), Error> {
        self.sender
            .send(command)
            .await
            .map_err(|err| anyhow!(err.to_string()))
    }
}

impl ChatServer {
    pub fn new(
        db_pool: Pool,
        config: Config,
        sender: mpsc::Sender<Command>,
        receiver: mpsc::Receiver<Command>,
    ) -> Self {
        ChatServer {
            db_pool,
            config,
            sender,
            receiver,
            connections: HashMap::new(),
            active_sessions: HashMap::new(),
            user_statuses: HashMap::new(),
        }
    }

    pub async fn run(mut self) {
        while let Some(cmd) = self.receiver.recv().await {
            use Command::*;

            match cmd {
                Send {
                    destination,
                    message,
                } => {
                    self.handle_message(destination, message).await;
                }
                Connect { socket } => {
                    self.handle_new_connection(socket).await;
                }
                Disconnect { session_id } => {
                    let connection_state = self.connections.remove(&session_id);

                    if let Some(connection_state) = connection_state {
                        let active_sessions = self
                            .active_sessions
                            .entry(connection_state.user.id)
                            .or_default();
                        active_sessions.remove(&session_id);

                        if active_sessions.is_empty() {
                            self.user_statuses.remove(&connection_state.user.id);
                        }

                        println!("User id={} disconnected", connection_state.user.id);
                    }
                }
            }
        }
    }

    async fn handle_new_connection(&mut self, socket: WebSocket) {
        let (auth_sender, mut auth_receiver) = mpsc::channel::<Result<UserAuthenticated, Error>>(1);

        let validate_token =
            ValidateTokenActorHandle::new(self.db_pool.clone(), self.config.clone());

        let mut connection =
            ConnectionHandle::new(socket, validate_token, auth_sender, self.sender.clone());

        let user_auth = tokio::spawn(async move {
            tokio::select! {
                recv = auth_receiver.recv() => {
                    if let Some(user_auth) = recv {
                        match user_auth {
                            Ok(user_auth) => Ok(Some(user_auth)),
                            Err(error) => Err(error),
                        }
                    } else {
                        Err(anyhow!("error receiving auth information"))
                    }

                },
                _ = tokio::time::sleep(Duration::from_secs(60)) => Ok(None),
            }
        })
        .await
        .unwrap();

        match user_auth {
            Ok(user_auth) => match user_auth {
                Some(user_auth) => {
                    self.connections.insert(
                        user_auth.session_id.clone(),
                        ConnectionState {
                            connection,
                            user: user_auth.user.clone(),
                        },
                    );
                    self.user_statuses
                        .insert(user_auth.user.id, UserStatus::Online);

                    let active_sessions =
                        self.active_sessions.entry(user_auth.user.id).or_default();
                    active_sessions.insert(user_auth.session_id.clone());

                    println!("User id={} connected", user_auth.user.id);
                }
                None => {
                    connection.close().await;
                }
            },
            Err(error) => {
                let _ = connection
                    .send_message(ServerEvent::AuthenticationError {
                        error: error.to_string(),
                    })
                    .await;
                connection.close().await;
            }
        }
    }

    async fn handle_message(&mut self, destination: User, message: ServerEvent) {
        let active_sessions = self.active_sessions.get(&destination.id);

        if active_sessions.is_none() {
            return;
        }

        for session in active_sessions.unwrap() {
            if let Some(state) = self.connections.get_mut(session) {
                state
                    .connection
                    .send_message(message.clone())
                    .await
                    .unwrap();
            }
        }
    }
}
