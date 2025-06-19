use std::sync::Arc;
use axum::{Form, Json as AxumJson, extract::{State, Path}, response::{Html, IntoResponse}, http::StatusCode};
use applicant::AppState;
use crate::models::{Event, EventStatus};
use job::models::Job;
use serde_json::{json, Value as Json};
use shared::IdToString;
use surrealdb::sql::Thing;
use chrono::NaiveDate;

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct EventForm {
    title: String,
    description: String,
    status: EventStatus,
    start_date: NaiveDate,
    end_date: NaiveDate,
    spotlight_job_id: Option<String>,
}
fn internal_error<E: std::fmt::Display>(e: E) -> (StatusCode, String) {
    (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
}

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
    Form(form): Form<EventForm>,
) -> impl IntoResponse {
    let spotlight_job_id = match form.spotlight_job_id.as_deref() {
        Some("") | None => None,
        Some(s) => Some(
            s.parse::<Thing>()
                .expect("frontend <select> always sends valid job:<id>")
        ),
    };

    let new_event = Event {
        id: None,
        title: form.title,
        description: form.description,
        status: form.status,
        start_date: form.start_date,
        end_date: form.end_date,
        spotlight_job_id,
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
