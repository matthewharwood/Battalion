use std::sync::Arc;
use axum::{Form, Router, routing::{get, post}, extract::State, response::{Html, IntoResponse}};
use schemars::schema_for;
use serde_json::Value;
use crate::{AppState, views};
use crate::models::apply::Apply;

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/apply", get(show_form).post(submit_form))
}

async fn show_form(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let schema = schema_for!(Apply);
    let schema_json: Value = serde_json::to_value(schema).unwrap();
    let tera = &state.views;
    let mut ctx = tera::Context::new();
    ctx.insert("schema", &schema_json);
    let rendered = tera.render("index.html.tera", &ctx).unwrap();
    Html(rendered)
}

async fn submit_form(State(state): State<Arc<AppState>>, Form(form): Form<Apply>) -> impl IntoResponse {
    match form.create(&state.db).await {
        Ok(_rec) => Html("Success".into()),
        Err(e) => {
            eprintln!("Failed to insert: {:?}", e);
            Html("Error".into())
        }
    }
}
