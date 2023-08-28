pub mod user;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

/// This allows easily converting [`std::error::Error`] errors into an axum
/// supported response, with an appropriate HTTP error code.
pub struct AppError {
    error: anyhow::Error,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, self.error.to_string()).into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(error: E) -> Self {
        Self {
            error: error.into(),
        }
    }
}
