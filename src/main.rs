use axum::{
    routing::{get, post},
    Router,
};
use controllers::user::create_user;

use std::net::SocketAddr;

pub mod database;
pub mod models;
pub mod schema;
pub mod views;
pub mod controllers;

#[tokio::main]
async fn main() {
    let pool = database::get_connection_pool().await
        .unwrap_or_else(|err| panic!("{}", err));

    let app = Router::new()
        .route("/", get(root))
        .route("/users", post(create_user))
        .with_state(pool);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}
