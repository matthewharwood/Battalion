use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum EventStatus {
    Scheduled,
    Lobby,
    Live,
    Ended,
    Archived,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Event {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub status: EventStatus,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub spotlight_job_id: Option<Uuid>,
}
