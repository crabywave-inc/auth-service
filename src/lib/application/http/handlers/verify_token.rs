use crate::application::http::handlers::{ApiError, ApiSuccess};
use crate::domain::auth::ports::AuthenticationService;
use axum::http::StatusCode;
use axum::{Extension, Json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize)]
pub struct VerifyTokenRequest {
    token: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct VerifyTokenResponse {
    email: String,
    id: String,
}

pub async fn verify_token<A: AuthenticationService>(
    Extension(authentication_service): Extension<Arc<A>>,
    Json(payload): Json<VerifyTokenRequest>,
) -> Result<ApiSuccess<VerifyTokenResponse>, ApiError> {
    authentication_service
        .verify_token(&payload.token)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))
        .map(|e| {
            ApiSuccess::new(
                StatusCode::OK,
                VerifyTokenResponse {
                    email: e.email,
                    id: e.id,
                },
            )
        })
}
