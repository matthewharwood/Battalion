use std::sync::Arc;
use axum::{Form, Json, extract::{State, Path}, response::{Html, IntoResponse}, http::StatusCode};
use chrono::Utc;
use serde_json::Value;
use shared::internal_error;
use crate::AppState;
use crate::models::Apply;

pub async fn show_form(
    State(state): State<Arc<AppState>>,
) -> Result<Html<String>, (StatusCode, String)> {
    let select_opts = state
        .db
        .query(
            "SELECT record::id(id) AS value, title as title, status as status, startDate as startDate FROM event
WHERE  status = $status
  AND  startDate >= $today
ORDER  BY startDate ASC
LIMIT  1;",
        ).bind(("status", "scheduled"))
        .bind(("today",  Utc::now().date_naive()))
        .await
        .map_err(internal_error)?
        .take::<Option<Value>>(0_usize)
        .map_err(internal_error)?;
    println!("{:?}", select_opts);
    let mut ctx = tera::Context::new();
    ctx.insert("data", &select_opts);

    let rendered = state.views.render("applicant_form.html", &ctx).unwrap();
    Ok(Html(rendered))
}

pub async fn submit_form(State(state): State<Arc<AppState>>, Form(form): Form<Apply>) -> impl IntoResponse {
    eprintln!("Received form data: {:?}", form);
    match form.create(&state.db).await {
        Ok(_rec) => Html(String::from("Success")),
        Err(e) => {
            eprintln!("Failed to insert: {:?}", e);
            Html(String::from("Error"))
        }
    }
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
