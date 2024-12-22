use crate::application::http::handlers::{ApiError, ApiSuccess};
use crate::domain::auth::ports::AuthenticationService;
use axum::http::StatusCode;
use axum::{Extension, Json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize)]
pub struct AuthRequest {
    email: String,
    password: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct LoginResponse {
    email: String,
    token: String,
}

pub async fn login<A: AuthenticationService>(
    Extension(authentication_service): Extension<Arc<A>>,
    Json(payload): Json<AuthRequest>,
) -> Result<ApiSuccess<LoginResponse>, ApiError> {
    authentication_service
        .login_user(&payload.email, &payload.password)
        .await
        .map_err(ApiError::from)
        .map(|token| {
            ApiSuccess::new(
                StatusCode::OK,
                LoginResponse {
                    email: payload.email,
                    token: token.token,
                },
            )
        })
}
