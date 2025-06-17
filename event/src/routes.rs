use axum::{Router, routing::{get, post}};
use crate::handlers;

pub fn router() -> Router {
    Router::new()
        .route("/events", get(handlers::list).post(handlers::create))
        .route("/events/new", get(handlers::new_form))
        .route("/events/:id/edit", get(handlers::edit))
        .route("/events/:id/delete", post(handlers::delete))
}
