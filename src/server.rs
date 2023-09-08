use std::net::SocketAddr;
use std::sync::Arc;

use anyhow::anyhow;
use tower::ServiceBuilder;

use crate::chat::server::ChatServerHandle;
use crate::config::Config;
use crate::database::{Pool, SqlxPool};
use crate::state::AppState;
use crate::{auth, router};

pub async fn serve(db_pool: Pool, sqlx_db_pool: SqlxPool) -> Result<(), anyhow::Error> {
    let (session_layer, auth_layer) = auth::create_auth(sqlx_db_pool.clone()).await?;

    let config = Config::init()?;
    let chat_server = ChatServerHandle::new();

    let state = Arc::new(AppState {
        config,
        db_pool,
        sqlx_db_pool,
        chat_server: Arc::from(chat_server),
    });

    let router = router::create_router(state.clone())
        .with_state(state.clone())
        .layer(ServiceBuilder::new().layer(session_layer).layer(auth_layer));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .map_err(|err| anyhow!(err.to_string()))
}
