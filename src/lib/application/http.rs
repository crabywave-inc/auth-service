pub mod handlers;
pub mod responses;

use crate::application::http::handlers::login::login;
use crate::application::http::handlers::verify_token::verify_token;
use crate::domain::auth::ports::AuthenticationService;
use anyhow::Context;
use axum::routing::{get, post};
use axum::{Extension, Router};
use handlers::health::{live, ready};
use std::sync::Arc;
use tokio::net;
use tracing::{info, info_span};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HttpServerConfig {
    pub port: String,
}

impl HttpServerConfig {
    pub fn new(port: String) -> Self {
        Self { port }
    }
}

#[derive(Debug, Clone)]
struct AppState<A>
where
    A: AuthenticationService,
{
    authentication_service: Arc<A>,
}

pub struct HttpServer {
    router: axum::Router,
    listener: net::TcpListener,
}

impl HttpServer {
    pub async fn new<A>(
        config: HttpServerConfig,
        authentication_service: Arc<A>,
    ) -> anyhow::Result<Self>
    where
        A: AuthenticationService,
    {
        let trace_layer = tower_http::trace::TraceLayer::new_for_http().make_span_with(
            |request: &axum::extract::Request| {
                let uri: String = request.uri().to_string();
                info_span!("http_request", method = ?request.method(), uri)
            },
        );

        let state = AppState {
            authentication_service,
        };

        let router = Router::new()
            .nest("", api_routes())
            .layer(trace_layer)
            .layer(Extension(Arc::clone(&state.authentication_service)))
            .with_state(state);

        let listener = net::TcpListener::bind(format!("0.0.0.0:{}", config.port))
            .await
            .with_context(|| format!("Failed to bind to port {}", config.port))?;

        Ok(Self { router, listener })
    }

    pub async fn run(self) -> anyhow::Result<()> {
        info!(
            "Server is running on http://{}",
            self.listener.local_addr()?
        );
        axum::serve(self.listener, self.router)
            .await
            .context("received error while running server")?;

        Ok(())
    }
}

fn api_routes<A>() -> axum::Router<AppState<A>>
where
    A: AuthenticationService,
{
    axum::Router::new()
        .route("/health/live", get(live))
        .route("/health/ready", get(ready))
        .route("/auth/login", post(login::<A>))
        .route("/auth/verify", post(verify_token::<A>))
}
