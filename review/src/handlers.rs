use std::sync::Arc;
use axum::{Form, Json, extract::{State, Path}, response::{Html, IntoResponse}, http::StatusCode};
use applicant::AppState;
use crate::models::Review;
use serde_json::Value;
use shared::internal_error;
use sha2::{Digest, Sha256};

pub(crate) async fn show_page(
    State(state): State<Arc<AppState>>
) -> Result<Html<String>, (StatusCode, String)> {
    let tera = &state.views;
    #[derive(serde::Serialize)]
    struct ScoreBox<'a> {
        count: u32,
        label: &'a str,
        class: &'a str,
    }



     #[derive(serde::Deserialize)]
    struct ScoreResult {
        yay_count: u64,
        may_count: u64,
        nay_count: u64,
    }

    // Query for YAY count (score = 1)
    let yay_count: u64 = state.db
        .query("SELECT count() AS count FROM vote_record WHERE score = 1 GROUP ALL;")
        .await
        .map_err(internal_error)?
        .take::<Vec<serde_json::Value>>(0)
        .map_err(internal_error)?
        .get(0)
        .and_then(|v| v.get("count"))
        .and_then(|v| v.as_u64())
        .unwrap_or(0);

    // Query for MAY count (score = 0)
    let may_count: u64 = state.db
        .query("SELECT count() AS count FROM vote_record WHERE score = 0 GROUP ALL;")
        .await
        .map_err(internal_error)?
        .take::<Vec<serde_json::Value>>(0)
        .map_err(internal_error)?
        .get(0)
        .and_then(|v| v.get("count"))
        .and_then(|v| v.as_u64())
        .unwrap_or(0);

    // Query for NAY count (score = -1)
    let nay_count: u64 = state.db
        .query("SELECT count() AS count FROM vote_record WHERE score = -1 GROUP ALL;")
        .await
        .map_err(internal_error)?
        .take::<Vec<serde_json::Value>>(0)
        .map_err(internal_error)?
        .get(0)
        .and_then(|v| v.get("count"))
        .and_then(|v| v.as_u64())
        .unwrap_or(0);

    println!("YAY count: {}, MAY count: {}, NAY count: {}", yay_count, may_count, nay_count);
    let scoreboard = vec![
        ScoreBox { count: yay_count as u32, label: "YAY", class: "yay" },
        ScoreBox { count: may_count as u32, label: "MAY", class: "may" },
        ScoreBox { count: nay_count as u32, label: "NAY", class: "nay" },
    ];

    let select_opts: Vec<Value> = state
        .db
        .query("SELECT string::concat('event:', record::id(id)) AS value, title as title FROM event;")
        .await
        .map_err(internal_error)?
        .take(0)
        .map_err(internal_error)?;

    let select_jobs: Vec<Value> = state
        .db
        .query("SELECT string::concat('job:', record::id(id)) AS value, title as title FROM job;")
        .await
        .map_err(internal_error)?
        .take(0)
        .map_err(internal_error)?;

    let select_applicants: Vec<Value> = state
        .db
        .query("SELECT string::concat('apply:', record::id(id)) AS value, title as title FROM apply;")
        .await
        .map_err(internal_error)?
        .take(0)
        .map_err(internal_error)?;

    let mut ctx = tera::Context::new();
    ctx.insert("scoreboard", &scoreboard);

    let first_event = select_opts
        .get(0)
        .and_then(|obj| obj.get("value"))
        .and_then(|val| val.as_str())
        .map(|s| s.to_string());

    let first_job = select_jobs
        .get(0)
        .and_then(|obj| obj.get("value"))
        .and_then(|val| val.as_str())
        .map(|s| s.to_string());

    let first_application = select_applicants.get(0)
    .and_then(|obj| obj.get("value"))
    .and_then(|val| val.as_str())
    .map(|s| s.to_string());

    let session_id = generate_session_id(first_application.as_deref(), first_event.as_deref(), first_job.as_deref());
    eprintln!("Generated session ID: {:?}", session_id);
    ctx.insert("event_id", &first_event);
    ctx.insert("applicant_id", &first_application);
    ctx.insert("session_id", &session_id);

    let rendered = tera.render("grid.html", &ctx).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Html(rendered))
}

pub(crate) async fn create_review(State(state): State<Arc<AppState>>, Form(form): Form<Review>) -> impl IntoResponse {
    match form.create(&state.db).await {
        Ok(_) => Html(String::from("Success")),
        Err(e) => {
            eprintln!("Failed to insert: {:?}", e);
            Html(String::from("Error"))
        }
    }
}

pub(crate) async fn fetch_review(State(state): State<Arc<AppState>>, Path(id): Path<String>) -> impl IntoResponse {
    match Review::get(&state.db, &id).await {
        Ok(Some(review)) => Json(review).into_response(),
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(e) => {
            eprintln!("Fetch error: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub(crate) async fn update_review(State(state): State<Arc<AppState>>, Path(id): Path<String>, Form(data): Form<Review>) -> impl IntoResponse {
    match Review::update(&state.db, &id, &data).await {
        Ok(Some(updated)) => Json(updated).into_response(),
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(e) => {
            eprintln!("Update error: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub(crate) async fn delete_review(State(state): State<Arc<AppState>>, Path(id): Path<String>) -> impl IntoResponse {
    match Review::delete(&state.db, &id).await {
        Ok(()) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => {
            eprintln!("Delete error: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

fn generate_session_id(a: Option<&str>, b: Option<&str>, c: Option<&str>) -> Option<String> {
    // Early return if any input is None
    let (a_val, b_val, c_val) = (a?, b?, c?);

    // Extract the parts after the colon
    let a_id = a_val.split_once(':').map(|(_, id)| id)?;
    let b_id = b_val.split_once(':').map(|(_, id)| id)?;
    let c_id = c_val.split_once(':').map(|(_, id)| id)?;

    // Concatenate
    let concatenated = format!("{}{}{}", a_id, b_id, c_id);

    // Hash using SHA256
    let mut hasher = Sha256::new();
    hasher.update(concatenated.as_bytes());
    let hash_result = hasher.finalize();

    // Convert hash to hex string
    let hash_hex = format!("{:x}", hash_result);

    // Return formatted output
    Some(format!("session:{}", hash_hex))
}