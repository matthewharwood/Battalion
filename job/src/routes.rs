use std::sync::Arc;
use axum::{Router, routing::get};
use applicant::AppState;
use crate::handlers;

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/jobs", get(handlers::show_form).post(handlers::submit_form))
        .route("/jobs/:id", get(handlers::fetch_form)
            .put(handlers::update_form)
            .delete(handlers::delete_form))
}
