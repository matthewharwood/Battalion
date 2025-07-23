use std::sync::Arc;
use axum::{extract::State, http::StatusCode, response::Html};
use tera::Context;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use surrealdb::Surreal;
use applicant::models::Apply;
use job::models::Job;
use shared::internal_error;
use crate::AppState;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaderboardEntry {
    pub name: String,
    pub email: String,
    pub total_score: i64,
    pub vote_count: i64,
    pub applicant_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaderboardStats {
    pub total_participants: i64,
    pub total_votes: i64,
    pub avg_score: f64,
}

pub async fn index(State(state): State<Arc<AppState>>) -> Result<Html<String>, (StatusCode, String)> {
    // Fetch leaderboard data
    let leaderboard_data = match fetch_leaderboard_data(&state.db).await {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error fetching leaderboard data: {}", e);
            Vec::new()
        }
    };
    
    // Fetch stats
    let stats = match fetch_leaderboard_stats(&state.db).await {
        Ok(stats) => stats,
        Err(e) => {
            eprintln!("Error fetching leaderboard stats: {}", e);
            LeaderboardStats {
                total_participants: 0,
                total_votes: 0,
                avg_score: 0.0,
            }
        }
    };

    // Fetch job options for select box
    let job_options = fetch_job_options(&state.db).await?;

    // Fetch all apply records for table display
    let apply_records = match fetch_apply_records(&state.db).await {
        Ok(records) => records,
        Err(e) => {
            eprintln!("Error fetching apply records: {}", e);
            Vec::new()
        }
    };

    let mut context = Context::new();
    // context.insert("leaderboard_data", &leaderboard_data);
    // context.insert("total_participants", &stats.total_participants);
    // context.insert("total_votes", &stats.total_votes);
    // context.insert("avg_score", &format!("{:.1}", stats.avg_score));
    eprintln!("Job options: {:?}", job_options);
    context.insert("job_options", &job_options);
    // context.insert("apply_records", &apply_records);

    let rendered = state
        .views
        .render("leaderboard.html", &context)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Html(rendered))
}

async fn fetch_leaderboard_data(db: &Surreal<surrealdb::engine::remote::ws::Client>) -> surrealdb::Result<Vec<LeaderboardEntry>> {
    // Query to aggregate votes by applicant and join with applicant data
    let query = "
        SELECT 
            applicant_id,
            math::sum(score) AS total_score,
            count() AS vote_count
        FROM vote_record 
        GROUP BY applicant_id 
        ORDER BY total_score DESC
        LIMIT 50
    ";
    
    let vote_aggregations: Vec<serde_json::Value> = db.query(query).await?.take(0)?;
    
    let mut leaderboard_entries = Vec::new();
    
    for vote_agg in vote_aggregations {
        let applicant_id = vote_agg["applicant_id"].as_str().unwrap_or("unknown");
        let total_score = vote_agg["total_score"].as_i64().unwrap_or(0);
        let vote_count = vote_agg["vote_count"].as_i64().unwrap_or(0);
        
        // Fetch applicant details
        let applicant_query = format!("SELECT * FROM apply WHERE id = {}", applicant_id);
        let applicants: Vec<Apply> = match db.query(&applicant_query).await {
            Ok(mut result) => result.take(0).unwrap_or_else(|_| Vec::new()),
            Err(_) => Vec::new(),
        };
        
        let (name, email) = if let Some(applicant) = applicants.first() {
            (applicant.name.clone(), applicant.email.clone())
        } else {
            ("Anonymous Applicant".to_string(), "No email provided".to_string())
        };
        
        leaderboard_entries.push(LeaderboardEntry {
            name,
            email,
            total_score,
            vote_count,
            applicant_id: applicant_id.to_string(),
        });
    }
    
    Ok(leaderboard_entries)
}

async fn fetch_leaderboard_stats(db: &Surreal<surrealdb::engine::remote::ws::Client>) -> surrealdb::Result<LeaderboardStats> {
    // Get total number of unique participants using SurrealDB syntax
    let participants_query = "SELECT array::len(array::distinct(applicant_id)) AS total FROM vote_record";
    let participants_result: Vec<serde_json::Value> = db.query(participants_query).await?.take(0)?;
    let total_participants = participants_result
        .first()
        .and_then(|v| v["total"].as_i64())
        .unwrap_or(0);
    
    // Get total votes and average score
    let votes_query = "SELECT count() AS total_votes, math::mean(score) AS avg_score FROM vote_record";
    let votes_result: Vec<serde_json::Value> = db.query(votes_query).await?.take(0)?;
    let vote_data = votes_result.first();
    
    let total_votes = vote_data
        .and_then(|v| v["total_votes"].as_i64())
        .unwrap_or(0);
    
    let avg_score = vote_data
        .and_then(|v| v["avg_score"].as_f64())
        .unwrap_or(0.0);
    
    Ok(LeaderboardStats {
        total_participants,
        total_votes,
        avg_score,
    })
}

async fn fetch_job_options(db: &Surreal<surrealdb::engine::remote::ws::Client>) -> Result<Vec<Value>, (StatusCode, String)> {
    let jobs: Vec<Value> = db
        .query("SELECT string::concat('job:', record::id(id)) AS value, title as title FROM job;")
        .await
        .map_err(internal_error)?
        .take(0)
        .map_err(internal_error)?;
    Ok(jobs)
}

async fn fetch_apply_records(db: &Surreal<surrealdb::engine::remote::ws::Client>) -> surrealdb::Result<Vec<Apply>> {
    let apply_records: Vec<Apply> = db.query("SELECT * FROM apply").await?.take(0)?;
    Ok(apply_records)
}