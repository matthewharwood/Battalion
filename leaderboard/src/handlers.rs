use std::sync::Arc;
use axum::{extract::State, http::StatusCode, response::Html};
use tera::Context;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use surrealdb::Surreal;
use shared::internal_error;
use crate::AppState;

#[derive(Deserialize, Serialize, Debug)]
struct VoteStats {
    id: String,
    name: String,
    job_id: String,
    yay_count: i64,
    nay_count: i64,
}

pub async fn index(State(state): State<Arc<AppState>>) -> Result<Html<String>, (StatusCode, String)> {
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

    // Fetch vote statistics
    let vote_stats = match fetch_vote_stats(&state.db).await {
        Ok(stats) => stats,
        Err(e) => {
            eprintln!("Error fetching vote stats: {}", e);
            Vec::new()
        }
    };

    let mut context = Context::new();
    context.insert("job_options", &job_options);
    context.insert("apply_records", &apply_records);
    context.insert("vote_stats", &vote_stats);

    eprintln!("vote_stats: {:?}", vote_stats);

    let rendered = state
        .views
        .render("leaderboard.html", &context)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Html(rendered))
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

async fn fetch_apply_records(db: &Surreal<surrealdb::engine::remote::ws::Client>) -> surrealdb::Result<Vec<Value>> {
    let apply_records: Vec<Value> = db.query("SELECT string::concat('apply:', record::id(id)) as id, name, email, phone, created_at FROM apply").await?.take(0)?;
    Ok(apply_records)
}

async fn fetch_vote_stats(db: &Surreal<surrealdb::engine::remote::ws::Client>) -> surrealdb::Result<Vec<VoteStats>> {
    // Convert the id to string to avoid serialization issues with Thing type
    let applicants_result = db.query("SELECT string::concat('apply:', record::id(id)) as id, name, job as job_id FROM apply ORDER BY name").await;
    
    let applicants: Vec<Value> = match applicants_result {
        Ok(mut response) => {
            match response.take(0) {
                Ok(apps) => apps,
                Err(e) => {
                    eprintln!("Error taking applicants result: {:?}", e);
                    return Ok(vec![]);
                }
            }
        },
        Err(e) => {
            eprintln!("Error querying applicants: {:?}", e);
            return Ok(vec![]);
        }
    };

    eprintln!("Found {} applicants", applicants.len());

    let mut results = vec![];

    for applicant in applicants {
        let id = applicant["id"].as_str().unwrap_or_default().to_string();
        let name = applicant["name"].as_str().unwrap_or_default().to_string();
        let job_id = applicant["job_id"].as_str().unwrap_or_default().to_string();
        
        eprintln!("Processing applicant: {} with id: {}", name, id);

        // Debug: Let's see what records exist for this applicant
        eprintln!("Looking for votes with applicantId = {}", id);
        
        // Count votes - select only score to avoid serialization issues with other fields
        let query_str = format!("SELECT score FROM vote_record WHERE applicantId = {} AND score = 1", id);
        eprintln!("Query: {}", query_str);
        
        let yay_result = db
            .query(&query_str)
            .await;
        
        let yay_count = match yay_result {
            Ok(mut response) => {
                match response.take::<Vec<Value>>(0) {
                    Ok(records) => records.len() as i64,
                    Err(e) => {
                        eprintln!("Error getting yay votes for {}: {:?}", name, e);
                        0
                    }
                }
            },
            Err(e) => {
                eprintln!("Error querying yay votes for {}: {:?}", name, e);
                0
            }
        };

        let nay_query_str = format!("SELECT score FROM vote_record WHERE applicantId = {} AND score = -1", id);
        eprintln!("Nay Query: {}", nay_query_str);
        
        let nay_result = db
            .query(&nay_query_str)
            .await;
        
        let nay_count = match nay_result {
            Ok(mut response) => {
                match response.take::<Vec<Value>>(0) {
                    Ok(records) => records.len() as i64,
                    Err(e) => {
                        eprintln!("Error getting nay votes for {}: {:?}", name, e);
                        0
                    }
                }
            },
            Err(e) => {
                eprintln!("Error querying nay votes for {}: {:?}", name, e);
                0
            }
        };

        results.push(VoteStats {
            id,
            name,
            job_id,
            yay_count,
            nay_count,
        });
    }

    // Sort by descending order of yay_count
    results.sort_by(|a, b| b.yay_count.cmp(&a.yay_count));

    Ok(results)
}
