use std::sync::Arc;
use axum::{Form, Json, extract::{State, Path, Query}, response::{Html, IntoResponse}, http::StatusCode};
use applicant::{AppState, models::Apply};
use job::models::Job;
use crate::models::Review;
use serde_json::Value;
use shared::internal_error;
use sha2::{Digest, Sha256};

#[derive(serde::Deserialize)]
pub struct ReviewQueryParams {
    applicant_id: Option<String>,
}

pub(crate) async fn show_page(
    State(state): State<Arc<AppState>>,
    Query(params): Query<ReviewQueryParams>
) -> Result<Html<String>, (StatusCode, String)> {
    let tera = &state.views;
    #[derive(serde::Serialize)]
    struct ScoreBox<'a> {
        count: u32,
        label: &'a str,
        class: &'a str,
    }

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

    // Get the applicant ID to filter votes by - either from params or use first applicant
    let target_applicant_id = params.applicant_id.as_ref()
        .map(|id| id.clone())
        .or_else(|| {
            // If no applicant_id in params, get the first applicant's ID
            select_applicants.get(0)
                .and_then(|obj| obj.get("value"))
                .and_then(|val| val.as_str())
                .map(|s| s.to_string())
        });

    let (yay_count, may_count, nay_count) = if let Some(applicant_id) = &target_applicant_id {
        // Query for YAY count (score = 1) for specific applicant
        let yay_query = format!("SELECT score FROM vote_record WHERE applicantId = {} AND score = 1", applicant_id);
        let yay_records: Vec<serde_json::Value> = state.db
            .query(&yay_query)
            .await
            .map_err(internal_error)?
            .take(0)
            .map_err(internal_error)?;
        let yay_count = yay_records.len() as u64;

        // Query for MAY count (score = 0) for specific applicant
        let may_query = format!("SELECT score FROM vote_record WHERE applicantId = {} AND score = 0", applicant_id);
        let may_records: Vec<serde_json::Value> = state.db
            .query(&may_query)
            .await
            .map_err(internal_error)?
            .take(0)
            .map_err(internal_error)?;
        let may_count = may_records.len() as u64;

        // Query for NAY count (score = -1) for specific applicant
        let nay_query = format!("SELECT score FROM vote_record WHERE applicantId = {} AND score = -1", applicant_id);
        let nay_records: Vec<serde_json::Value> = state.db
            .query(&nay_query)
            .await
            .map_err(internal_error)?
            .take(0)
            .map_err(internal_error)?;
        let nay_count = nay_records.len() as u64;

        (yay_count, may_count, nay_count)
    } else {
        // Fallback to 0 if no applicant found
        (0, 0, 0)
    };

    println!("YAY count: {}, MAY count: {}, NAY count: {}", yay_count, may_count, nay_count);
    let scoreboard = vec![
        ScoreBox { count: yay_count as u32, label: "YAY", class: "yay" },
        ScoreBox { count: may_count as u32, label: "MAY", class: "may" },
        ScoreBox { count: nay_count as u32, label: "NAY", class: "nay" },
    ];

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

    // Fetch the applicant record based on the provided ID or get the first one
    let first_applicant: Option<Apply> = if let Some(applicant_id) = &params.applicant_id {
        // Use the full applicant_id as provided (should be "apply:xxxxx")
        let query = format!("SELECT * FROM apply WHERE id = {}", applicant_id);
        eprintln!("Querying for applicant with ID: {}", applicant_id);
        
        state.db
            .query(&query)
            .await
            .map_err(internal_error)?
            .take(0)
            .map_err(internal_error)?
    } else {
        // Fallback to first applicant if no ID provided
        state.db
            .query("SELECT * FROM apply LIMIT 1")
            .await
            .map_err(internal_error)?
            .take(0)
            .map_err(internal_error)?
    };

    eprintln!("first_applicant ID: {:?}", first_applicant);

    // Get the job record using job_id from first_applicant
    let job_record = if let Some(ref applicant) = first_applicant {
        if let Some(ref job_thing) = applicant.job {
            // Extract just the ID part from the Thing
            let job_id_str = job_thing.id.to_string();
            match Job::get(&state.db, &job_id_str).await {
                Ok(job_opt) => {
                    eprintln!("Job record: {:?}", job_opt);
                    job_opt
                }
                Err(e) => {
                    eprintln!("Failed to get job: {:?}", e);
                    None
                }
            }
        } else {
            None
        }
    } else {
        None
    };

    if let Some(ref job) = job_record {
        ctx.insert("job", job);
    }

    ctx.insert("applicant", &first_applicant);
    ctx.insert("yay_count", &yay_count);
    ctx.insert("nay_count", &nay_count);

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