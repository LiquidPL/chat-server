use std::sync::Arc;

use axum::{
    middleware,
    routing::{delete, get, post},
    Router, http::HeaderValue,
};

use hyper::header;
use tower_http::{
    cors::{Any, CorsLayer},
    services::ServeDir,
};

use crate::controllers::{
    channel::{create_channel, delete_channel, get_channel},
    message::{create_message, delete_message, get_messages},
    user::{create_user, current_user, login, logout},
    websocket::open_websocket,
};
use crate::{auth::jwt_auth, state::AppState};

pub fn create_router(state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .route("/channels/:channel_id", get(get_channel))
        .route("/channels", post(create_channel))
        .route("/channels/:channel_id", delete(delete_channel))
        .route("/channels/:channel_id/messages", get(get_messages))
        .route("/channels/:channel_id/messages", post(create_message))
        .route("/users/me", get(current_user))
        .route("/users/logout", post(logout))
        .route(
            "/channels/:channel_id/messages/:message_id",
            delete(delete_message),
        )
        .route_layer(middleware::from_fn_with_state(state, jwt_auth))
        .route("/websocket", get(open_websocket))
        .route("/users", post(create_user))
        .route("/users/login", post(login))
        .route_layer(
            CorsLayer::new()
                .allow_methods(Any)
                .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
                .allow_headers([header::AUTHORIZATION, header::CONTENT_TYPE]),
        )
        .nest_service("/", ServeDir::new("web/out"))
}
