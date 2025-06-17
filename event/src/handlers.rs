use std::sync::Arc;
use axum::{Form, Json, extract::{State, Path}, http::StatusCode, response::{Html, IntoResponse}};
use schemars::schema_for;
use serde_json::Value;
use tera::Context;
use shared::AppState;
use crate::models::Event;

pub async fn show_form(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let schema = schema_for!(Event);
    let schema_json: Value = serde_json::to_value(schema).unwrap();
    let mut ctx = Context::new();
    ctx.insert("schema", &schema_json);
    let rendered = state
        .views
        .render("event/templates/event_form.html", &ctx)
        .unwrap();
    Html(rendered)
}

pub async fn submit_form(State(state): State<Arc<AppState>>, Form(form): Form<Event>) -> impl IntoResponse {
    match form.create(&state.db).await {
        Ok(_) => Html(String::from("Success")),
        Err(e) => {
            eprintln!("Failed to insert: {:?}", e);
            Html(String::from("Error"))
        }
    }
}

pub async fn fetch(State(state): State<Arc<AppState>>, Path(id): Path<String>) -> impl IntoResponse {
    match Event::get(&state.db, &id).await {
        Ok(Some(event)) => Json(event).into_response(),
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(e) => {
            eprintln!("Fetch error: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn update(State(state): State<Arc<AppState>>, Path(id): Path<String>, Form(data): Form<Event>) -> impl IntoResponse {
    match Event::update(&state.db, &id, &data).await {
        Ok(Some(updated)) => Json(updated).into_response(),
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(e) => {
            eprintln!("Update error: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn delete(State(state): State<Arc<AppState>>, Path(id): Path<String>) -> impl IntoResponse {
    match Event::delete(&state.db, &id).await {
        Ok(()) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => {
            eprintln!("Delete error: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
