use std::sync::Arc;

use axum::{
    middleware,
    routing::{delete, get, post},
    Router,
};
use tower_http::services::ServeDir;

use crate::{auth::jwt_auth, state::AppState};
use crate::controllers::{
        channel::{create_channel, delete_channel, get_channel},
        message::{create_message, delete_message},
        user::{create_user, current_user, login, logout},
        websocket::open_websocket,
    };

pub fn create_router(state: Arc<AppState>) -> Router<Arc<AppState>> {
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
        .route_layer(middleware::from_fn_with_state(state, jwt_auth))
        // .route_layer(RequireAuthorizationLayer::<UserId, User>::login())
        .route("/users", post(create_user))
        .route("/users/login", post(login))
        .nest_service("/", ServeDir::new("static"))
}
