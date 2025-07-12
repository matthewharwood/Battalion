use std::sync::Arc;
use axum::{extract::State, http::StatusCode, response::Html};
use tera::Context;
use crate::AppState;

pub async fn index(State(state): State<Arc<AppState>>) -> Result<Html<String>, (StatusCode, String)> {
    let rendered = state
        .views
        .render("queue.html", &Context::new())
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Html(rendered))
}