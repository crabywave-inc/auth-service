use auth_service::application::http::{HttpServer, HttpServerConfig};
use auth_service::env::Env;
use auth_service::infrastructure::auth::firebase::authentication_service::AuthenticationServiceFirebase;
use clap::Parser;
use std::sync::Arc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    let env = Arc::new(Env::parse());

    let authentication_service = Arc::new(AuthenticationServiceFirebase::new(
        env.firebase_api_key.clone(),
    ));

    let server_config = HttpServerConfig::new(env.port.clone());

    let http_server = HttpServer::new(server_config, Arc::clone(&authentication_service)).await?;

    http_server.run().await?;

    Ok(())
}
