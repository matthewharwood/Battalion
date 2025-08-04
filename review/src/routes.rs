use std::sync::Arc;
use axum::{Router, routing::get};
use applicant::AppState;
use crate::handlers;

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/reviews", get(handlers::show_page))
        .route("/review/{id}", get(handlers::fetch_review)
            .put(handlers::update_review)
            .delete(handlers::delete_review))
}
