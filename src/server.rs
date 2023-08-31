use std::net::SocketAddr;
use std::sync::Arc;

use anyhow::anyhow;
use tower::ServiceBuilder;

use crate::chat::server::ChatServer;
use crate::database::{Pool, SqlxPool};
use crate::state::AppState;
use crate::{auth, router};

pub async fn serve(db_pool: Pool, sqlx_db_pool: SqlxPool) -> Result<(), anyhow::Error> {
    let (session_layer, auth_layer) = auth::create_auth(sqlx_db_pool.clone()).await?;

    let chat_server = ChatServer::new().start();

    let router = router::create_router()
        .with_state(Arc::new(AppState {
            db_pool,
            sqlx_db_pool,
            chat_server: Arc::from(chat_server),
        }))
        .layer(ServiceBuilder::new().layer(session_layer).layer(auth_layer));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .map_err(|err| anyhow!(err.to_string()))
}
