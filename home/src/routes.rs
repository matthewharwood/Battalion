use std::sync::Arc;
use axum::{Router, routing::get};
use crate::handlers;
use crate::AppState;

pub fn routes() -> Router<Arc<AppState>> {
    Router::new().route("/", get(handlers::index))
}