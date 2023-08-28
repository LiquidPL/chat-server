use axum::{routing::post, Router};

use crate::controllers::user::{create_user, login, logout};
use crate::server::AppState;

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/users", post(create_user))
        .route("/users/login", post(login))
        .route("/users/logout", post(logout))
}
