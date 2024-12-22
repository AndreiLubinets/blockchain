use axum::response::IntoResponse;
use tracing::error;
use reqwest::StatusCode;

pub enum ApiError {
    BadRequest(String),
    InternalServerError(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        match self {
            ApiError::BadRequest(error) => (StatusCode::BAD_REQUEST, error).into_response(),
            ApiError::InternalServerError(error) => {
                error!("{}", &error);
                (StatusCode::INTERNAL_SERVER_ERROR, error).into_response()
            }
        }
    }
}

impl<E> From<E> for ApiError
where
    E: std::error::Error,
{
    fn from(error: E) -> Self {
        ApiError::InternalServerError(error.to_string())
    }
}
