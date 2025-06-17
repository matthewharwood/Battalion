use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum JobLevel {
    Intern,
    Junior,
    Mid,
    Senior,
    Staff,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Job {
    pub id: Uuid,
    pub title: String,
    pub level: JobLevel,
    pub team: String,
    pub location: String,
    pub description: String,
}
