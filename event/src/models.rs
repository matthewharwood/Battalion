use serde::{Deserialize, Serialize};
use serde_with::{serde_as, FromInto};
use serde_withs::ThingString;
use surrealdb::{Surreal, engine::remote::ws::Client as WsClient, sql::Thing};
use chrono::{DateTime, Utc};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EventStatus {
    Pending, 
    Scheduled,
    Lobby,
    Live,
    Ended,
    Archived,
}

#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    pub id: Option<Thing>,
    pub title: String,
    pub description: String,
    pub status: EventStatus,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    #[serde_as(as = "Option<FromInto<ThingString>>")]
    pub job: Option<Thing>,
}

impl Event {
    pub async fn create(self, db: &Surreal<WsClient>) -> surrealdb::Result<Self> {
        println!("Creating event: {:?}", self);
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
