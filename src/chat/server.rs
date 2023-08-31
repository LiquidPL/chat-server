use std::{collections::HashMap, ops::ControlFlow};

use axum::extract::ws::{Message, WebSocket};
use futures::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use tokio::sync::mpsc;

use crate::{
    models::user::{User, UserId},
    views::chat::UserStatus,
};

pub struct ChatServer {
    tx: Option<mpsc::Sender<Command>>,
}

pub enum Command {
    Send { destination: User, message: String },
    Connect { user: User, socket: WebSocket },
    Disconnect(User),
}

impl ChatServer {
    pub fn new() -> Self {
        ChatServer { tx: None }
    }

    pub fn start(mut self) -> Self {
        if self.tx.is_some() {
            panic!("this function can be only called once per instance");
        }

        let (tx, rx) = mpsc::channel::<Command>(32);

        self.tx = Some(tx);

        tokio::spawn(Self::manager_loop(
            self.tx
                .as_ref()
                .expect("tx was initialized two lines above")
                .clone(),
            rx,
        ));

        self
    }

    pub fn get_manager_tx(&self) -> mpsc::Sender<Command> {
        let tx = self.tx.as_ref().expect("ChatServer is not initialized");

        tx.clone()
    }

    async fn manager_loop(tx: mpsc::Sender<Command>, mut rx: mpsc::Receiver<Command>) {
        let mut senders: HashMap<UserId, SplitSink<WebSocket, Message>> = HashMap::new();
        let mut user_statuses: HashMap<UserId, UserStatus> = HashMap::new();

        while let Some(cmd) = rx.recv().await {
            use Command::*;

            match cmd {
                Send {
                    destination,
                    message,
                } => {
                    if let Some(sender) = senders.get_mut(&destination.id) {
                        sender.send(Message::Text(message)).await.unwrap();
                    }
                }
                Connect { user, socket } => {
                    let (sender, receiver) = socket.split();

                    senders.insert(user.id, sender);
                    user_statuses.insert(user.id, UserStatus::Online);

                    println!("User {} connected", user.username);

                    Self::connection_loop(user, tx.clone(), receiver).await;
                }
                Disconnect(user) => {
                    let _ = senders.remove(&user.id).unwrap();
                    user_statuses.remove(&user.id).unwrap();
                    println!("User {} disconnected", user.username);
                }
            }
        }
    }

    async fn connection_loop(
        user: User,
        tx: mpsc::Sender<Command>,
        mut receiver: SplitStream<WebSocket>,
    ) {
        tokio::spawn(async move {
            while let Some(Ok(msg)) = receiver.next().await {
                if Self::process_message(msg).is_break() {
                    tx.send(Command::Disconnect(user)).await.unwrap();
                    break;
                }
            }
        });
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
