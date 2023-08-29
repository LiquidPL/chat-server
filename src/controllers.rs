pub mod channel;
pub mod user;
pub mod websocket;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

/// This allows easily converting [`std::error::Error`] errors into an axum
/// supported response, with an appropriate HTTP error code.
pub struct AppError {
    status_code: StatusCode,
    error: anyhow::Error,
}

impl AppError {
    fn new(error: anyhow::Error) -> Self {
        Self {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            error,
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (self.status_code, self.error.to_string()).into_response()
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
