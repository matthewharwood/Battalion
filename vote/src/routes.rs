use std::sync::Arc;
use axum::{Router, routing::{post, get}};
use applicant::AppState;
use crate::handlers;

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/vote", post(handlers::submit_vote))
        .route("/rpc", get(handlers::rpc_handler))
}