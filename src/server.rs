use std::net::SocketAddr;

use anyhow::anyhow;
use tower::ServiceBuilder;

use crate::database::{Pool, SqlxPool};
use crate::{auth, router};

#[derive(Clone)]
pub struct AppState {
    pub db_pool: Pool,
    pub sqlx_db_pool: SqlxPool,
}

pub async fn serve(db_pool: Pool, sqlx_db_pool: SqlxPool) -> Result<(), anyhow::Error> {
    let (session_layer, auth_layer) = auth::create_auth(sqlx_db_pool.clone()).await?;

    let router = router::create_router()
        .with_state(AppState {
            db_pool,
            sqlx_db_pool,
        })
        .layer(ServiceBuilder::new().layer(session_layer).layer(auth_layer));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .map_err(|err| anyhow!(err.to_string()))
}
