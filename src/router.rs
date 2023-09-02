use std::sync::Arc;

use axum::{
    routing::{delete, get, post},
    Router,
};
use axum_login::RequireAuthorizationLayer;
use tower_http::services::ServeDir;

use crate::state::AppState;
use crate::{
    controllers::{
        channel::{get_channel, create_channel, delete_channel},
        user::{create_user, login, logout, current_user},
        message::{create_message, delete_message},
        websocket::open_websocket,
    },
    models::user::{User, UserId},
};

pub fn create_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/websocket", get(open_websocket))
        .route("/channels/:channel_id", get(get_channel))
        .route("/channels", post(create_channel))
        .route("/channels/:channel_id", delete(delete_channel))
        .route("/channels/:channel_id/messages", post(create_message))
        .route("/users/me", get(current_user))
        .route("/users/logout", post(logout))
        .route(
            "/channels/:channel_id/messages/:message_id",
            delete(delete_message),
        )
        .route_layer(RequireAuthorizationLayer::<UserId, User>::login())
        .route("/users", post(create_user))
        .route("/users/login", post(login))
        .nest_service("/", ServeDir::new("static"))
}
