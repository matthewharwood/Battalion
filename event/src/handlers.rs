use std::sync::Arc;
use axum::{extract::{Path, State}, response::{Html, IntoResponse}, Form, Json as AxumJson, http::StatusCode};
use applicant::AppState;
use crate::models::Event;
use serde_json::Value as Json;
use shared::internal_error;


pub async fn show_form(State(state): State<Arc<AppState>>) -> Result<Html<String>, (StatusCode, String)> {

    let select_opts: Vec<Json> = state
        .db
        .query("SELECT record::id(id) AS value, title as title FROM job;")
        .await
        .map_err(internal_error)?
        .take::<Vec<Json>>(0)
        .map_err(internal_error)?;

    let mut ctx = tera::Context::new();
    ctx.insert("job_options", &select_opts);
    let rendered = state.views.render("event_form.html", &ctx).unwrap();
    Ok(Html(rendered))
}

pub(crate) async fn submit_form(
    State(state): State<Arc<AppState>>,
    Form(event): Form<Event>,
) -> impl IntoResponse {
    let new_event = Event {
        id: None,
        ..event
    };

    match new_event.create(&state.db).await {
        Ok(_) => Html(String::from("Success")),
        Err(e) => {
            eprintln!("Failed to insert event: {e:?}");
            Html(String::from("Error"))
        }
    }
}

pub(crate) async fn fetch_form(State(state): State<Arc<AppState>>, Path(id): Path<String>) -> impl IntoResponse {
    match Event::get(&state.db, &id).await {
        Ok(Some(event)) => AxumJson(event).into_response(),
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(e) => {
            eprintln!("Fetch error: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub(crate) async fn update_form(State(state): State<Arc<AppState>>, Path(id): Path<String>, Form(data): Form<Event>) -> impl IntoResponse {
    match Event::update(&state.db, &id, &data).await {
        Ok(Some(updated)) => AxumJson(updated).into_response(),
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
