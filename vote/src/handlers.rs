use std::sync::Arc;
use axum::{
    extract::{Json, State, ws::{WebSocket, WebSocketUpgrade, Message}},
    response::IntoResponse,
    http::StatusCode,
};
use futures::StreamExt;
use chrono::Utc;
use surrealdb::{Surreal, engine::remote::ws::Client as WsClient};
use applicant::AppState; // ✅ Correct cross-crate import
use crate::models::{IncomingVote, VoteRecord};
use crate::schema::Post;

// HTTP vote submit handler
pub(crate) async fn submit_vote(
    State(state): State<Arc<AppState>>,
    Json(data): Json<IncomingVote>,
) -> impl IntoResponse {
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
                Ok(_) => StatusCode::CREATED.into_response(),
                Err(e) => {
                    eprintln!("DB insert error: {:?}", e);
                    StatusCode::INTERNAL_SERVER_ERROR.into_response()
                }
            }
        }
        Err(e) => {
            eprintln!("DB read error: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

// WebSocket live handler
pub async fn rpc_handler(
    State(app_state): State<Arc<AppState>>,
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    let db = app_state.db.clone();
    ws.on_upgrade(move |socket| handle_ws(socket, db))
}

async fn handle_ws(mut socket: WebSocket, db: Arc<Surreal<WsClient>>) {
    let mut stream = match db.select::<Vec<Post>>("posts").live().await {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Surreal live query error: {:?}", e);
            return;
        }
    };

    while let Some(Ok(notification)) = stream.next().await {
        let txt = serde_json::to_string(&(notification.action, notification.data)).unwrap();
        if socket.send(Message::Text(txt.into())).await.is_err() {
            break;
        }
    }
}
