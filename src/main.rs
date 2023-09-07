pub mod auth;
pub mod chat;
pub mod config;
pub mod controllers;
pub mod database;
pub mod models;
pub mod router;
pub mod schema;
pub mod server;
pub mod state;
pub mod views;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let db_pool = database::create_connection_pool()
        .await
        .unwrap_or_else(|err| panic!("{}", err));

    let sqlx_db_pool = database::create_sqlx_pool()
        .await
        .unwrap_or_else(|err| panic!("{}", err));

    server::serve(db_pool, sqlx_db_pool)
        .await
        .unwrap_or_else(|err| panic!("Error while running the server: {}", err.to_string()));

    Ok(())
}
