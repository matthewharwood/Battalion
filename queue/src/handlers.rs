use std::sync::Arc;
use axum::{extract::State, http::StatusCode, response::Html};
use tera::Context;
use crate::AppState;
use shared::internal_error;
use serde_json::Value;
use chrono::{DateTime, Utc};

pub async fn index(State(state): State<Arc<AppState>>) -> Result<Html<String>, (StatusCode, String)> {
    let select_opts: Vec<Value> = state
        .db
        .query("SELECT string::concat('event:', record::id(id)) AS value, title as title, startDate AS startDate FROM event;")
        .await
        .map_err(internal_error)?   
        .take(0)                    
        .map_err(internal_error)?; 

    let mut ctx = Context::new();
    
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

    let rendered = state
        .views
        .render("queue.html", &ctx)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Html(rendered))
}