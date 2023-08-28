use std::{sync::Arc, ops::ControlFlow};

use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::IntoResponse, Extension,
};
use futures::{sink::SinkExt, stream::StreamExt};

use crate::{state::AppState, views::chat::UserStatus, models::user::User};

pub async fn open_websocket(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state, user))
}

async fn handle_socket(
    socket: WebSocket,
    state: Arc<AppState>,
    user: User,
) {
    let (mut sender, mut receiver) = socket.split();

    on_connect(&state, &user);

    sender.send(Message::Text(String::from("hello"))).await.unwrap();

    tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            if process_message(msg).is_break() {
                on_disconnect(&state, &user);
                break;
            }
        }
    });
}

fn on_connect(state: &AppState, user: &User) {
    let mut user_status = state.user_status.lock().unwrap();
    user_status.insert(user.id, UserStatus::Online);

    println!("User {} connected", user.username);
}

fn on_disconnect(state: &AppState, user: &User) {
    let mut user_status = state.user_status.lock().unwrap();
    user_status.remove(&user.id);

    println!("User {} disconnected", user.username);
}

fn process_message(msg: Message) -> ControlFlow<(), ()> {
    match msg {
        Message::Text(t) => println!("{}", t),
        Message::Close(_) => {
            return ControlFlow::Break(());
        },
        _ => ()
    }
    ControlFlow::Continue(())
}
