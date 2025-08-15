use std::sync::Arc;
use axum::{extract::State, response::Html, http::StatusCode};
use crate::AppState;
use tera::Context;

pub async fn error_404(State(state): State<Arc<AppState>>) -> Result<Html<String>, (StatusCode, String)> {
    let mut ctx = Context::new();
    ctx.insert("error_code", "404");
    ctx.insert("error_message", "Page Not Found");
    ctx.insert("error_description", "The page you are looking for does not exist.");

    let rendered = state
        .views
        .render("error.html", &ctx)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Html(rendered))
}