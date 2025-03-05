use axum::extract::State;
use axum::Json;
use axum::response::IntoResponse;
use crate::infra::dependencies::AppState;


pub async fn health(
    State(state): State<AppState>
) -> Json<String> {
    tracing::info!("health");
    Json("ok".to_string())
}