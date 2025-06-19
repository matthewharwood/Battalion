use std::sync::Arc;
use axum::{Form, extract::State, response::IntoResponse, http::StatusCode};
use chrono::Utc;
use crate::models::{IncomingVote, VoteRecord};
use applicant::AppState;

pub(crate) async fn submit_vote(
    State(state): State<Arc<AppState>>,
    Form(data): Form<IncomingVote>,
) -> impl IntoResponse {
    // Check if vote already exists
    let existing: surrealdb::Result<Option<VoteRecord>> = state.db
        .query("SELECT * FROM vote_record WHERE applicant_id = $a AND session_id = $s LIMIT 1")
        .bind(("a", data.applicant_id.clone()))
        .bind(("s", data.session_id.clone()))
        .await
        .and_then(|mut r| r.take(0));

    match existing {
        Ok(Some(_)) => StatusCode::CONFLICT.into_response(),
        Ok(None) => {
            let record = VoteRecord {
                id: None,
                applicant_id: data.applicant_id,
                event_id: data.event_id,
                session_id: data.session_id,
                score: data.score,
                timestamp: Utc::now(),
            };
            match record.create(&state.db).await {
                Ok(_r) => StatusCode::CREATED.into_response(),
                Err(_e) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
            }
        }
        Err(_e) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
