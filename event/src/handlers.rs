use std::sync::Arc;
use axum::{Form, Json, extract::{State, Path}, response::{Html, IntoResponse}, http::StatusCode};
use applicant::AppState;
use crate::models::Event;

pub(crate) async fn show_form(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let tera = &state.views;
    let ctx = tera::Context::new();
    let rendered = tera.render("event_form.html", &ctx).unwrap();
    Html(rendered)
}

pub(crate) async fn submit_form(State(state): State<Arc<AppState>>, Form(form): Form<Event>) -> impl IntoResponse {
    match form.create(&state.db).await {
        Ok(_) => Html(String::from("Success")),
        Err(e) => {
            eprintln!("Failed to insert: {:?}", e);
            Html(String::from("Error"))
        }
    }
}

pub(crate) async fn fetch_form(State(state): State<Arc<AppState>>, Path(id): Path<String>) -> impl IntoResponse {
    match Event::get(&state.db, &id).await {
        Ok(Some(event)) => Json(event).into_response(),
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(e) => {
            eprintln!("Fetch error: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub(crate) async fn update_form(State(state): State<Arc<AppState>>, Path(id): Path<String>, Form(data): Form<Event>) -> impl IntoResponse {
    match Event::update(&state.db, &id, &data).await {
        Ok(Some(updated)) => Json(updated).into_response(),
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(e) => {
            eprintln!("Update error: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub(crate) async fn delete_form(State(state): State<Arc<AppState>>, Path(id): Path<String>) -> impl IntoResponse {
    match Event::delete(&state.db, &id).await {
        Ok(()) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => {
            eprintln!("Delete error: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
