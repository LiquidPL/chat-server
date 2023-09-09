use std::sync::Arc;

use crate::{chat::server::Command, state::AppState};
use axum::{
    extract::{
        ws::{WebSocket, WebSocketUpgrade},
        State,
    },
    response::IntoResponse,
};

pub async fn open_websocket(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: Arc<AppState>) {
    state
        .chat_server
        .send_command(Command::Connect { socket })
        .await
        .unwrap();
}
