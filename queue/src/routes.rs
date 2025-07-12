use std::sync::Arc;
use axum::{Router, routing::get};
use crate::{handlers, AppState};

pub fn routes() -> Router<Arc<AppState>> {
    Router::new().route("/queue", get(handlers::index))
}