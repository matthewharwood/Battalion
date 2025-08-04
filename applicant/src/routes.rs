use std::sync::Arc;
use axum::{Router, routing::get};
use crate::{AppState, handlers};

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/apply", get(handlers::show_form).post(handlers::submit_form))
        .route("/apply/{job_id}", get(handlers::apply_redirect))

}
