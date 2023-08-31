use std::sync::Arc;

use crate::{chat::server::Command, models::user::User, state::AppState};
use axum::{
    extract::{
        ws::{WebSocket, WebSocketUpgrade},
        State,
    },
    response::IntoResponse,
    Extension,
};

pub async fn open_websocket(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state, user))
}

async fn handle_socket(socket: WebSocket, state: Arc<AppState>, user: User) {
    let tx = state.chat_server.get_manager_tx();

    tx.send(Command::Connect {
        user: user.clone(),
        socket,
    })
    .await
    .unwrap();
}
