use std::net::SocketAddr;
use axum::Router;
use clap::Parser;
use tracing_subscriber::fmt::format::FmtSpan;
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
    let args = Args::parse();
    let config = Config::new(args.config_path.as_str(),args.secret_path.as_str());

    tracing_subscriber::fmt()
        .with_env_filter(&config.app().log_level)
        .with_span_events(FmtSpan::FULL)
        .with_file(true)
        .with_target(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .init();


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
