use std::sync::Arc;
use axum::{Form, extract::{State }, response::{Html}, http::StatusCode};
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
        .map_err(internal_error)?   
        .take(0)                    
        .map_err(internal_error)?; 

    // Step 2: Create Tera context and insert event options
    let mut ctx = Context::new();
    eprintln!("select_opts{:?}", select_opts);
    ctx.insert("event_options", &select_opts);

    if let Some(Value::Object(first)) = select_opts.get(0) {
        if let Some(Value::String(start_date)) = first.get("startDate") {
            if let Ok(datetime) = start_date.parse::<DateTime<Utc>>() {
                // Get Unix timestamp in seconds
                let timestamp = datetime.timestamp();
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

    Ok(Html(rendered))
}

pub async fn apply(
    State(state): State<Arc<AppState>>,
    Form(form_data): Form<ApplicationForm>,
) -> Result<Html<String>, (StatusCode, String)> {
    eprintln!("Application form submitted: {:?}", form_data);
    let mut ctx = Context::new();
    ctx.insert("event_id", &form_data.event_id);
    // Redirect to job_form.html
    let rendered = state
        .views
        .render("job_form.html", &ctx)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Html(rendered))
}