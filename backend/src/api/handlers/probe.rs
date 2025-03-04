use axum::extract::State;
use axum::response::IntoResponse;
use axum::Router;
use axum::routing::get;
use crate::infra::dependencies::AppState;

async fn routes() -> Router<AppState> {
    Router::new()
        .route("/health", get(health))
}


async fn health(
    State(state): State<AppState>
) -> impl IntoResponse {
    Ok(())
}