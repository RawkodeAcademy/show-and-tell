use axum::response::{IntoResponse, Response};
use reqwest::StatusCode;

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
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}
