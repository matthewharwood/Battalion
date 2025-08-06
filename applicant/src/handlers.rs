use std::sync::Arc;
use axum::{Form, Json, extract::{State, Path}, response::{Html, IntoResponse, Redirect}, http::StatusCode};
use chrono::Utc;
use shared::{internal_error, IdToString};
use crate::AppState;
use crate::models::Apply;
use serde_json::Value;
use tera::Context;

pub async fn show_form(
    State(state): State<Arc<AppState>>
) -> Result<Html<String>, (StatusCode, String)> {
    
    // Query events
    let select_opts: Vec<Value> = state
        .db
        .query("SELECT string::concat('event:', record::id(id)) AS value, title as title FROM event;")
        .await
        .map_err(internal_error)?
        .take(0)
        .map_err(internal_error)?;

    // Query jobs
    let select_jobs: Vec<Value> = state
        .db
        .query("SELECT string::concat('job:', record::id(id)) AS value, title as title FROM job;")
        .await
        .map_err(internal_error)?
        .take(0)
        .map_err(internal_error)?;

    // Build context
    let mut ctx = Context::new();
    ctx.insert("event_options", &select_opts);
    ctx.insert("job_options", &select_jobs);

    println!("event_options -----------> {:?}", &select_opts);

    let rendered = state.views.render("applicant_form.html", &ctx)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Html(rendered))
}

pub async fn submit_form(State(state): State<Arc<AppState>>, Form(form): Form<Apply>) -> impl IntoResponse {
    println!("=== APPLICANT FORM SUBMISSION ===");
    println!("Form data: {:?}", form);
    match form.create(&state.db).await {
        Ok(created_app) => {
            eprintln!("created_app: {:?}", created_app);
            // Create review record after successful application creation
            if let (Some(app_id), Some(event), Some(_job)) = (created_app.id.as_ref(), created_app.event.as_ref(), created_app.job.as_ref()) {
                // Create vote_record after successful application creation
                let vote_query = "CREATE vote_record CONTENT {
                    applicant_id: $applicant_id,
                    name: $applicant_name,
                    event_id: $event_id,
                    session_id: $session_id,
                    score: 0,
                    timestamp: $timestamp
                }";
                
                // Generate a simple session ID using timestamp and applicant name
                let session_id = format!("session:{}", Utc::now().timestamp_millis());
                let current_timestamp = Utc::now();
                
                if let Err(e) = state.db.query(vote_query)
                    .bind(("applicant_id", app_id.clone()))
                    .bind(("applicant_name", created_app.name.clone()))
                    .bind(("event_id", event.clone()))
                    .bind(("session_id", session_id))
                    .bind(("timestamp", current_timestamp))
                    .await {
                    eprintln!("Failed to create vote_record: {:?}", e);
                    // Continue anyway - application was created successfully
                }
            }
            
            // Return the application ID as JSON so frontend can store it
            let response_data = serde_json::json!({
                "success": true,
                "application_id": created_app.id_string(),
                "redirect": "/queue"
            });
            Json(response_data).into_response()
        },
        Err(e) => {
            eprintln!("Failed to insert: {:?}", e);
            let error_response = serde_json::json!({
                "success": false,
                "error": "Failed to submit application"
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)).into_response()
        }
    }
}

/// Redirects old style `/apply/{job_id}` paths to `/apply?job_id=`.
pub async fn apply_redirect(Path(job_id): Path<String>) -> Redirect {
    Redirect::temporary(&format!("/apply?job_id={}", job_id))
}

pub async fn fetch_form(State(state): State<Arc<AppState>>, Path(id): Path<String>) -> impl IntoResponse {
    match Apply::get(&state.db, &id).await {
        Ok(Some(app)) => Json(app).into_response(),
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(e) => {
            eprintln!("Fetch error: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn update_form(State(state): State<Arc<AppState>>, Path(id): Path<String>, Form(data): Form<Apply>) -> impl IntoResponse {
    match Apply::update(&state.db, &id, &data).await {
        Ok(Some(updated)) => Json(updated).into_response(),
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(e) => {
            eprintln!("Update error: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn delete_form(State(state): State<Arc<AppState>>, Path(id): Path<String>) -> impl IntoResponse {
    match Apply::delete(&state.db, &id).await {
        Ok(()) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => {
            eprintln!("Delete error: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
