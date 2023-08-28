pub mod database;
pub mod schema;
pub mod models;
pub mod router;
pub mod controllers;
pub mod views;
pub mod server;


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let db_pool = database::create_connection_pool()
        .await
        .unwrap_or_else(|err| panic!("{}", err));

    server::serve(db_pool).await
        .unwrap_or_else(|err| panic!("Error while running the server: {}", err.to_string()));

    Ok(())
}
