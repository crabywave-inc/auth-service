use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum AuthenticationError {
    #[error("User not found")]
    UserNotFound,
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("Internal server error {0}")]
    InternalServerError(String),
}

#[derive(Debug)]
pub struct AuthLoginResponse {
    pub display_name: String,
    pub email: String,
    pub token: String,
}

pub struct AuthVerifyTokenResponse {
    pub email: String,
    pub display_name: String,
    pub id: String,
}

impl AuthLoginResponse {
    pub fn new(display_name: String, email: String, token: String) -> Self {
        Self {
            display_name,
            email,
            token,
        }
    }
}

impl AuthVerifyTokenResponse {
    pub fn new(email: String, display_name: String, id: String) -> Self {
        Self {
            email,
            display_name,
            id,
        }
    }
}
