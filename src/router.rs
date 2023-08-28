use std::sync::Arc;

use axum::{routing::{get, post}, Router};
use axum_login::RequireAuthorizationLayer;
use tower_http::services::ServeDir;

use crate::{controllers::user::{create_user, login, logout}, models::user::{UserId, User}};
use crate::controllers::websocket::open_websocket;
use crate::state::AppState;

pub fn create_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/websocket", get(open_websocket))
        .route_layer(RequireAuthorizationLayer::<UserId, User>::login())
        .route("/users", post(create_user))
        .route("/users/login", post(login))
        .route("/users/logout", post(logout))
        .nest_service("/", ServeDir::new("static"))
}
