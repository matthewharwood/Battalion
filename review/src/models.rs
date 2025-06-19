use serde::{Deserialize, Serialize};
use surrealdb::{Surreal, engine::remote::ws::Client as WsClient, sql::Thing};
use shared::impl_id_to_string_for;   // bring the macro into scope


#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Review {
    pub id: Option<Thing>,
    pub event: Thing,
    pub applicant: Thing,
    pub score: i8,
    pub comment: Option<String>,
}

impl_id_to_string_for!(Review);

impl Review {
    pub async fn create(self, db: &Surreal<WsClient>) -> surrealdb::Result<Self> {
        let created: Option<Self> = db.create("review").content(self).await?;
        Ok(created.expect("create returned none"))
    }

    pub async fn get(db: &Surreal<WsClient>, id: &str) -> surrealdb::Result<Option<Self>> {
        db.select(("review", id)).await
    }

    pub async fn update(db: &Surreal<WsClient>, id: &str, data: &Self) -> surrealdb::Result<Option<Self>> {
        db.update(("review", id)).content(data.clone()).await
    }

    pub async fn delete(db: &Surreal<WsClient>, id: &str) -> surrealdb::Result<()> {
        db.delete(("review", id)).await.map(|_: Option<Self>| ())
    }
}

