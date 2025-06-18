use std::sync::Arc;
use axum::{Router, routing::get};
use crate::handlers;
use shared::AppState;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/events", get(handlers::show_form).post(handlers::submit_form))
        .route(
            "/events/{id}",
            get(handlers::fetch)
                .put(handlers::update)
                .delete(handlers::delete),
        )
}
