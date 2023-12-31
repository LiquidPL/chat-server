pub mod channel;
pub mod message;
pub mod user;
pub mod websocket;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;

/// This allows easily converting [`std::error::Error`] errors into an axum
/// supported response, with an appropriate HTTP error code.
pub struct AppError {
    pub status_code: StatusCode,
    pub error: anyhow::Error,
}

impl AppError {
    pub fn new(error: anyhow::Error) -> Self {
        Self {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            error,
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            self.status_code,
            json!({"error": self.error.to_string()}).to_string(),
        )
            .into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(error: E) -> Self {
        AppError::new(error.into())
    }
}
