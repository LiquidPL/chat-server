use std::{collections::HashMap, sync::Mutex};

use crate::{database::{Pool, SqlxPool}, models::user::UserId, views::chat::UserStatus};

pub struct AppState {
    pub db_pool: Pool,
    pub sqlx_db_pool: SqlxPool,
    pub user_status: Mutex<HashMap<UserId, UserStatus>>,
}
