use std::sync::Arc;
use axum::{Router, routing::get};
use applicant::AppState;
use crate::handlers;

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/review", get(handlers::show_page).post(handlers::create_review))
        .route("/review/{id}", get(handlers::fetch_review)
            .put(handlers::update_review)
            .delete(handlers::delete_review))
}
