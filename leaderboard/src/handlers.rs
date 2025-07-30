use std::sync::Arc;
use axum::{extract::State, http::StatusCode, response::Html};
use tera::Context;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use surrealdb::{Surreal, sql::Thing};
use applicant::models::Apply;
use job::models::Job;
use event::models::Event;
use shared::internal_error;
use chrono::Utc;
use crate::AppState;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaderboardEntry {
    pub name: String,
    pub email: String,
    pub total_score: i64,
    pub vote_count: i64,
    pub yay_count: i64,
    pub nay_count: i64,
    pub applicant_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaderboardStats {
    pub total_participants: i64,
    pub total_votes: i64,
    pub avg_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoteRecord {
    pub applicant_id: Thing,
    pub score: i64,
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
    context.insert("leaderboard_data", &leaderboard_data);
    context.insert("total_participants", &stats.total_participants);
    context.insert("total_votes", &stats.total_votes);
    context.insert("avg_score", &format!("{:.1}", stats.avg_score));
    // eprintln!("Job options: {:?}", job_options);
    eprintln!("leaderboard_data: {:?}", leaderboard_data);
    context.insert("job_options", &job_options);
    context.insert("apply_records", &apply_records);

    let rendered = state
        .views
        .render("leaderboard.html", &context)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Html(rendered))
}

async fn fetch_leaderboard_data(db: &Surreal<surrealdb::engine::remote::ws::Client>) -> surrealdb::Result<Vec<LeaderboardEntry>> {
    // First, get all applicants
    let all_applicants: Vec<Apply> = db.query("SELECT * FROM apply").await?.take(0)?;
    
    // Get all vote records using custom struct
    let all_votes: Vec<VoteRecord> = db.query("SELECT applicant_id, score FROM vote_record").await?.take(0)?;
    
    let mut leaderboard_entries = Vec::new();
    
    // Process all applicants, including those with no votes
    for applicant in all_applicants {
        let applicant_id = if let Some(id) = &applicant.id {
            format!("apply:{}", id.id)
        } else {
            continue; // Skip applicants without IDs
        };
        
        // Calculate vote statistics for this applicant
        let mut total_score = 0i64;
        let mut vote_count = 0i64;
        let mut yay_count = 0i64;
        let mut nay_count = 0i64;
        
        for vote in &all_votes {
            let vote_applicant_id = format!("{}:{}", vote.applicant_id.tb, vote.applicant_id.id);
            if vote_applicant_id == applicant_id {
                total_score += vote.score;
                vote_count += 1;
                if vote.score == 1 {
                    yay_count += 1;
                } else if vote.score == -1 {
                    nay_count += 1;
                }
            }
        }
        
        leaderboard_entries.push(LeaderboardEntry {
            name: applicant.name.clone(),
            email: applicant.email.clone(),
            total_score,
            vote_count,
            yay_count,
            nay_count,
            applicant_id: applicant_id,
        });
    }
    
    // Sort by total_score descending, then by name ascending for consistent ordering
    leaderboard_entries.sort_by(|a, b| {
        b.total_score.cmp(&a.total_score)
            .then_with(|| a.name.cmp(&b.name))
    });
    
    // Limit to 50 entries
    leaderboard_entries.truncate(50);
    
    Ok(leaderboard_entries)
}

async fn fetch_leaderboard_stats(db: &Surreal<surrealdb::engine::remote::ws::Client>) -> surrealdb::Result<LeaderboardStats> {
    // Get all vote records to calculate stats manually using custom struct
    let all_votes: Vec<VoteRecord> = db.query("SELECT applicant_id, score FROM vote_record").await?.take(0)?;
    
    let total_votes = all_votes.len() as i64;
    
    // Count unique participants
    let mut unique_participants = std::collections::HashSet::new();
    let mut total_score = 0i64;
    
    for vote in &all_votes {
        let vote_applicant_id = format!("{}:{}", vote.applicant_id.tb, vote.applicant_id.id);
        unique_participants.insert(vote_applicant_id);
        total_score += vote.score;
    }
    
    let total_participants = unique_participants.len() as i64;
    let avg_score = if total_votes > 0 {
        total_score as f64 / total_votes as f64
    } else {
        0.0
    };
    
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