use chrono::NaiveDate;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use surrealdb::{Surreal, engine::remote::ws::Client as WsClient};

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub enum EventStatus {
    Scheduled,
    Lobby,
    Live,
    Ended,
    Archived,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct Event {
    #[serde(default = "Uuid::new_v4")]
    #[schemars(with = "String")]
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub status: EventStatus,
    #[schemars(with = "String")]
    pub start_date: NaiveDate,
    #[schemars(with = "String")]
    pub end_date: NaiveDate,
    #[schemars(with = "String")] 
    pub spotlight_job_id: Option<Uuid>,
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
