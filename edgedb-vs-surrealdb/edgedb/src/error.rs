use axum::response::{IntoResponse, Response};
use reqwest::StatusCode;
use tracing::error;

pub struct ApplicationError(anyhow::Error);

impl<E> From<E> for ApplicationError
where
    E: Into<anyhow::Error>,
{
    fn from(error: E) -> Self {
        Self(error.into())
    }
}

impl IntoResponse for ApplicationError {
    fn into_response(self) -> Response {
        error!("Application error: {}", self.0);

        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    }
}
