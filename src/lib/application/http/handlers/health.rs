use super::{ApiError, ApiSuccess};
use axum::http::StatusCode;

pub async fn live() -> Result<ApiSuccess<String>, ApiError> {
    Ok(ApiSuccess::new(StatusCode::OK, "I'm alive".to_string()))
}

pub async fn ready() -> Result<ApiSuccess<String>, ApiError> {
    Ok(ApiSuccess::new(StatusCode::OK, "I'm ready".to_string()))
}