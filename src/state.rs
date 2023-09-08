use std::sync::Arc;

use crate::{
    chat::server::ChatServerHandle,
    config::Config,
    database::{Pool, SqlxPool},
};

pub struct AppState {
    pub config: Config,
    pub db_pool: Pool,
    pub sqlx_db_pool: SqlxPool,
    pub chat_server: Arc<ChatServerHandle>,
}
