use std::net::SocketAddr;

use crate::router;
use crate::database::Pool;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: Pool,
}

pub async fn serve(db_pool: Pool) -> Result<(), hyper::Error> {
    let router = router::create_router()
        .with_state(AppState { db_pool });

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
}
