use std::sync::Arc;
use axum::{Form, Json, extract::{State, Path}, response::{Html, IntoResponse}, http::StatusCode};
// use axum::{extract::State, response::Html, http::StatusCode};
use crate::AppState;
use shared::internal_error;
use serde_json::Value;
use tera::Context;
use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ApplicationForm {
    event_id: String,
}


pub async fn index(State(state): State<Arc<AppState>>) -> Result<Html<String>, (StatusCode, String)> {
    // Step 1: Query event options from the SurrealDB database
    let select_opts: Vec<Value> = state
        .db
        .query("SELECT string::concat('event:', record::id(id)) AS value, title as title, startDate AS startDate FROM event;")
        .await
        .map_err(internal_error)?         // Convert DB error to standardized response
        .take(0)                          // Get the first query result set
        .map_err(internal_error)?;        // Convert deserialization error if any

    // Step 2: Create Tera context and insert event options
    // eprintln!("Hello, world! {:?}", select_opts);
    let mut ctx = Context::new();
    ctx.insert("event_options", &select_opts);

    if let Some(Value::Object(first)) = select_opts.get(0) {
        if let Some(Value::String(start_date)) = first.get("startDate") {
            // Parse the ISO 8601 date string into a chrono DateTime<Utc>
            if let Ok(datetime) = start_date.parse::<DateTime<Utc>>() {
                let timestamp = datetime.timestamp(); // Unix timestamp in seconds
                // eprintln!("Unix timestamp: {}", timestamp);
                ctx.insert("time_stamp", &timestamp);
            } else {
                eprintln!("Failed to parse datetime");
            }
        }
    }


    // Step 3: Render the `index.html` template with the populated context
    let rendered = state
        .views
        .render("index.html", &ctx)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Step 4: Return the rendered HTML wrapped in `Ok`
    Ok(Html(rendered))
}

pub async fn apply(
    State(state): State<Arc<AppState>>,
    Form(form_data): Form<ApplicationForm>,
) -> Result<Html<String>, (StatusCode, String)> {
    eprintln!("Application form submitted: {:?}", form_data);

    // Optionally, process or store form_data here

    // Render a confirmation template (recommended UX)
    let mut ctx = Context::new();
    ctx.insert("event_id", &form_data.event_id);

    let rendered = state
        .views
        .render("application_submitted.html", &ctx)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Html(rendered))
}