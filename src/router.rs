use std::sync::Arc;

use axum::{
    routing::{delete, get, post},
    Router,
};
use axum_login::RequireAuthorizationLayer;
use tower_http::services::ServeDir;

use crate::controllers::{
    channel::{delete_channel, get_channel},
    message::{create_message, delete_message},
    websocket::open_websocket,
};
use crate::state::AppState;
use crate::{
    controllers::{
        channel::create_channel,
        user::{create_user, login, logout},
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
        .route(
            "/channels/:channel_id/messages/:message_id",
            delete(delete_message),
        )
        .route_layer(RequireAuthorizationLayer::<UserId, User>::login())
        .route("/users", post(create_user))
        .route("/users/login", post(login))
        .route("/users/logout", post(logout))
        .nest_service("/", ServeDir::new("static"))
}
