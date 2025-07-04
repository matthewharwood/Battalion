use std::sync::Arc;
use axum::{extract::State, response::Html, http::StatusCode};
use crate::AppState;
use shared::internal_error;

pub async fn index(State(state): State<Arc<AppState>>) -> Result<Html<String>, (StatusCode, String)> {
    state
        .views
        .render("index.html", &tera::Context::new())
        .map(Html)
        .map_err(internal_error)
}