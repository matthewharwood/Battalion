use axum::{Router, routing::{get, post}};
use crate::handlers;

pub fn router() -> Router {
    Router::new()
        .route("/applicants", get(handlers::list).post(handlers::create))
        .route("/applicants/new", get(handlers::new_form))
        .route("/applicants/{id}/edit", get(handlers::edit))
        .route("/applicants/{id}/delete", post(handlers::delete))
}
