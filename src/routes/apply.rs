use std::sync::Arc;
use axum::{Form, Router, routing::{get, post, put, delete}, extract::{State, Path}, response::{Html, IntoResponse}, Json, http::StatusCode};
use schemars::schema_for;
use serde_json::Value;
use crate::AppState;
use crate::models::apply::Apply;

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/apply", get(show_form).post(submit_form))
        .route("/apply/:id", get(fetch_form).put(update_form).delete(delete_form))
}

async fn show_form(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let schema = schema_for!(Apply);
    let schema_json: Value = serde_json::to_value(schema).unwrap();
    let tera = &state.views;
    let mut ctx = tera::Context::new();
    ctx.insert("schema", &schema_json);
    let rendered = tera.render("index.html", &ctx).unwrap();
    Html(rendered)
}

async fn submit_form(State(state): State<Arc<AppState>>, Form(form): Form<Apply>) -> impl IntoResponse {
    match form.create(&state.db).await {
        Ok(_rec) => Html(String::from("Success")),
        Err(e) => {
            eprintln!("Failed to insert: {:?}", e);
            Html(String::from("Error"))
        }
    }
}

async fn fetch_form(State(state): State<Arc<AppState>>, Path(id): Path<String>) -> impl IntoResponse {
    match Apply::get(&state.db, &id).await {
        Ok(Some(app)) => Json(app).into_response(),
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(e) => {
            eprintln!("Fetch error: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

async fn update_form(State(state): State<Arc<AppState>>, Path(id): Path<String>, Json(data): Json<Apply>) -> impl IntoResponse {
    match Apply::update(&state.db, &id, &data).await {
        Ok(Some(updated)) => Json(updated).into_response(),
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(e) => {
            eprintln!("Update error: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

async fn delete_form(State(state): State<Arc<AppState>>, Path(id): Path<String>) -> impl IntoResponse {
    match Apply::delete(&state.db, &id).await {
        Ok(()) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => {
            eprintln!("Delete error: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
