use crate::domain::auth::entities::{
    AuthLoginResponse, AuthVerifyTokenResponse, AuthenticationError,
};
use crate::domain::auth::ports::AuthenticationService;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct AuthenticationServiceFirebase {
    api_key: String,
    http_client: Client,
}

impl AuthenticationServiceFirebase {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            http_client: Client::new(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FirebaseLoginResponse {
    #[serde(rename = "displayName")]
    display_name: String,
    email: String,
    #[serde(rename = "expiresIn")]
    expires_in: String,
    #[serde(rename = "idToken")]
    id_token: String,
    kind: String,
    #[serde(rename = "localId")]
    local_id: String,
    #[serde(rename = "refreshToken")]
    refresh_token: String,
    registered: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FirebaseUser {
    #[serde(rename = "createdAt")]
    created_at: String,
    disabled: bool,
    email: String,
    #[serde(rename = "emailVerified")]
    email_verified: bool,
    #[serde(rename = "lastLoginAt")]
    last_login_at: String,
    #[serde(rename = "lastRefreshAt")]
    last_refresh_at: String,
    #[serde(rename = "localId")]
    local_id: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FirebaseVerifyTokenResponse {
    kind: String,
    users: Vec<FirebaseUser>,
}

impl AuthenticationService for AuthenticationServiceFirebase {
    async fn login_user(
        &self,
        email: &str,
        password: &str,
    ) -> Result<AuthLoginResponse, AuthenticationError> {
        let url = format!(
            "https://identitytoolkit.googleapis.com/v1/accounts:signInWithPassword?key={}",
            self.api_key
        );

        let response = self
            .http_client
            .post(&url)
            .json(&serde_json::json!({
                "email": email,
                "password": password,
                "returnSecureToken": true
            }))
            .send()
            .await
            .map_err(|_| AuthenticationError::InternalServerError)?;

        let t = response
            .json::<FirebaseLoginResponse>()
            .await
            .map_err(|_| AuthenticationError::InternalServerError)?;

        let auth_login_response = AuthLoginResponse::new(t.display_name, t.email, t.id_token);

        Ok(auth_login_response)
    }

    async fn verify_token(
        &self,
        token: &str,
    ) -> Result<AuthVerifyTokenResponse, AuthenticationError> {
        let url = format!(
            "https://identitytoolkit.googleapis.com/v1/accounts:lookup?key={}",
            self.api_key
        );

        let response = self
            .http_client
            .post(&url)
            .json(&serde_json::json!({
                "idToken": token
            }))
            .send()
            .await
            .map_err(|_| AuthenticationError::InternalServerError)?;

        let body = response
            .json::<FirebaseVerifyTokenResponse>()
            .await
            .map_err(|_| AuthenticationError::InternalServerError)?;

        let auth_verify_token_response =
            AuthVerifyTokenResponse::new(body.users[0].email.clone(), body.users[0].email.clone());

        Ok(auth_verify_token_response)
    }
}
