use axum::Router;
use axum::routing::get;
use crate::infra::dependencies::AppState;

pub fn route() -> Router<AppState> {
    Router::new()
        .route("/health", get(crate::api::handlers::probe::health))
}
