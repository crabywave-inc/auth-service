use auth_service::application::http::{HttpServer, HttpServerConfig};
use auth_service::env::{AppEnv, Env};
use auth_service::infrastructure::auth::firebase::authentication_service::AuthenticationServiceFirebase;
use clap::Parser;
use std::sync::Arc;

fn init_logger(env: Arc<Env>) {
    match env.env {
        AppEnv::Development => {
            tracing_subscriber::fmt::init();
        }
        AppEnv::Production => {
            tracing_subscriber::fmt()
            .json()
            .with_max_level(tracing::Level::INFO)
            .init();
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    let env = Arc::new(Env::parse());

    //tracing_subscriber::fmt::init();
    init_logger(Arc::clone(&env));



    let authentication_service = Arc::new(AuthenticationServiceFirebase::new(
        env.firebase_api_key.clone(),
    ));

    let server_config = HttpServerConfig::new(env.port.clone());

    let http_server = HttpServer::new(server_config, Arc::clone(&authentication_service)).await?;

    http_server.run().await?;

    Ok(())
}
