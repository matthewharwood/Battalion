use std::sync::Arc;
use axum::{Router, routing::{get, post, put, delete}};
use crate::handlers;
use shared::AppState;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/jobs", get(handlers::show_form).post(handlers::submit_form))
        .route(
            "/jobs/:id",
            get(handlers::fetch)
                .put(handlers::update)
                .delete(handlers::delete),
        )
}
