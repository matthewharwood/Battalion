use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use surrealdb::{Surreal, engine::remote::ws::Client as WsClient, sql::Thing};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum EventStatus {
    Pending,
    Scheduled,
    Live,
    Ended,
    Archived,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    pub id: Option<Thing>,
    pub title: String,
    pub description: String,
    pub status: EventStatus,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub spotlight_job_id: Option<Thing>,
}

impl Event {
    pub async fn create(self, db: &Surreal<WsClient>) -> surrealdb::Result<Self> {
        let created: Option<Self> = db.create("event").content(self).await?;
        Ok(created.expect("create returned none"))
    }

    pub async fn get(db: &Surreal<WsClient>, id: &str) -> surrealdb::Result<Option<Self>> {
        db.select(("event", id)).await
    }

    pub async fn update(db: &Surreal<WsClient>, id: &str, data: &Self) -> surrealdb::Result<Option<Self>> {
        db.update(("event", id)).content(data.clone()).await
    }

    pub async fn delete(db: &Surreal<WsClient>, id: &str) -> surrealdb::Result<()> {
        db.delete(("event", id)).await.map(|_: Option<Self>| ())
    }
}
