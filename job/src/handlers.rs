use std::sync::Arc;
use axum::{
    extract::{Path, State},
    response::{Html, IntoResponse},
    Form, Json,
    http::StatusCode,
};
use applicant::AppState;
use crate::models::Job;
use tera::{Context};

pub(crate) async fn show_form(State(state): State<Arc<AppState>>) -> Result<impl IntoResponse, (StatusCode, &'static str)> {
    let tera = &state.views;
    let db = &state.db;

    let jobs: Vec<Job> = db
        .query("SELECT * FROM job")
        .await
        .map_err(|e| {
            eprintln!("Query error: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Query failed")
        })?
        .take(0)
        .map_err(|e| {
            eprintln!("Deserialization error: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Deserialization failed")
        })?;

    println!("Jobs: {:?}", jobs);

    // let jobs_json = serde_json::to_value(&jobs).unwrap();
    // println!("Jobs as JSON: {}", jobs_json);
    
    let mut ctx = Context::new();
    ctx.insert("jobs", &jobs);

    tera.render("job_form.html", &ctx)
        .map(Html)
        .map_err(|e| {
            eprintln!("Template rendering error: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Template rendering failed")
        })
}

pub(crate) async fn submit_form(State(state): State<Arc<AppState>>, Form(form): Form<Job>) -> impl IntoResponse {
    match form.create(&state.db).await {
        Ok(_) => Html(String::from("Success")),
        Err(e) => {
            eprintln!("Failed to insert: {:?}", e);
            Html(String::from("Error"))
        }
    }
}

pub(crate) async fn fetch_form(State(state): State<Arc<AppState>>, Path(id): Path<String>) -> impl IntoResponse {
    match Job::get(&state.db, &id).await {
        Ok(Some(job)) => Json(job).into_response(),
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(e) => {
            eprintln!("Fetch error: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub(crate) async fn update_form(State(state): State<Arc<AppState>>, Path(id): Path<String>, Form(data): Form<Job>) -> impl IntoResponse {
    match Job::update(&state.db, &id, &data).await {
        Ok(Some(updated)) => Json(updated).into_response(),
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(e) => {
            eprintln!("Update error: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub(crate) async fn delete_form(State(state): State<Arc<AppState>>, Path(id): Path<String>) -> impl IntoResponse {
    match Job::delete(&state.db, &id).await {
        Ok(()) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => {
            eprintln!("Delete error: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
