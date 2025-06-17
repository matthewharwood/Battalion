use axum::{Router, routing::{get, post}};
use crate::handlers;

pub fn router() -> Router {
    Router::new()
        .route("/jobs", get(handlers::list).post(handlers::create))
        .route("/jobs/new", get(handlers::new_form))
        .route("/jobs/{id}/edit", get(handlers::edit))
        .route("/jobs/{id}/delete", post(handlers::delete))
}
