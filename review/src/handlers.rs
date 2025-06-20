use std::sync::Arc;
use axum::{Form, Json, extract::{State, Path}, response::{Html, IntoResponse}, http::StatusCode};
use applicant::AppState;
use crate::models::Review;

pub(crate) async fn show_page(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let tera = &state.views;
    #[derive(serde::Serialize)]
    struct ScoreBox<'a> {
        count: u32,
        label: &'a str,
        class: &'a str,
    }

    let scoreboard = vec![
        ScoreBox { count: 2,  label: "YAY", class: "yay" },
        ScoreBox { count: 0,  label: "MAY", class: "may" },
        ScoreBox { count: 12, label: "NAY", class: "nay" },
    ];

    let mut ctx = tera::Context::new();
    ctx.insert("scoreboard", &scoreboard);
    // schema removed
    let rendered = tera.render("grid.html", &ctx).unwrap();
    Html(rendered)
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
