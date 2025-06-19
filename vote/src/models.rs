use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::{Surreal, engine::remote::ws::Client as WsClient, sql::Thing};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VoteRecord {
    pub id: Option<Thing>,
    pub applicant_id: Thing,
    pub event_id: Thing,
    pub session_id: Thing,
    pub score: i8,
    pub timestamp: DateTime<Utc>,
}

impl VoteRecord {
    pub async fn create(self, db: &Surreal<WsClient>) -> surrealdb::Result<Self> {
        let created: Option<Self> = db.create("vote_record").content(self).await?;
        Ok(created.expect("create returned none"))
    }

    pub async fn get(db: &Surreal<WsClient>, id: &str) -> surrealdb::Result<Option<Self>> {
        db.select(("vote_record", id)).await
    }

    pub async fn update(db: &Surreal<WsClient>, id: &str, data: &Self) -> surrealdb::Result<Option<Self>> {
        db.update(("vote_record", id)).content(data.clone()).await
    }

    pub async fn delete(db: &Surreal<WsClient>, id: &str) -> surrealdb::Result<()> {
        db.delete(("vote_record", id)).await.map(|_: Option<Self>| ())
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IncomingVote {
    pub applicant_id: Thing,
    pub event_id: Thing,
    pub session_id: Thing,
    pub score: i8,
}
