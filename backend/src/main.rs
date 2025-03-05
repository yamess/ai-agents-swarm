use std::net::SocketAddr;
use axum::Router;
use clap::Parser;
use crate::infra::cli::Args;
use crate::infra::config::Config;
use crate::infra::dependencies::AppState;

mod domain;
mod infra;
mod utils;
mod app;
mod api;
mod tables;
mod prelude;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt::init();

    let args = Args::parse();

    let config = Config::new(
        args.config_path.as_str(),
         args.secret_path.as_str()
        );
    let app_state = AppState::new(config.clone());

    let router = Router::new()
        .merge(api::routers::probe::route())
        .with_state(app_state);

    let addr = format!("{}:{}", config.app().host, config.app().port).parse::<SocketAddr>()
        .unwrap();

    tracing::info!("Listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;

    axum::serve(listener, router).await
}
