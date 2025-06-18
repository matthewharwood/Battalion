use std::sync::Arc;
use axum::{Router, routing::get};
use crate::handlers;
use shared::AppState;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/apply", get(handlers::show_form).post(handlers::submit_form))
        .route(
            "/apply/{id}",
            get(handlers::fetch_form)
                .put(handlers::update_form)
                .delete(handlers::delete_form),
        )
}
