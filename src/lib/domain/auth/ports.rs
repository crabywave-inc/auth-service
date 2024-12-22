use crate::domain::auth::entities::{
    AuthLoginResponse, AuthVerifyTokenResponse, AuthenticationError,
};
use std::future::Future;

pub trait AuthenticationService: Clone + Send + Sync + 'static {
    fn login_user(
        &self,
        email: &str,
        password: &str,
    ) -> impl Future<Output = Result<AuthLoginResponse, AuthenticationError>> + Send;
    fn verify_token(
        &self,
        token: &str,
    ) -> impl Future<Output = Result<AuthVerifyTokenResponse, AuthenticationError>> + Send;
}
