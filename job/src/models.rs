use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use surrealdb::{Surreal, engine::remote::ws::Client as WsClient};

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub enum JobLevel {
    Intern,
    Junior,
    Mid,
    Senior,
    Staff,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct Job {
    #[serde(default = "Uuid::new_v4")]
    #[schemars(with = "String")]
    pub id: Uuid,
    pub title: String,
    pub level: JobLevel,
    pub team: String,
    pub location: String,
    pub description: String,
}

impl Job {
    pub async fn create(self, db: &Surreal<WsClient>) -> surrealdb::Result<Self> {
        let created: Option<Self> = db.create("job").content(self).await?;
        Ok(created.expect("create returned none"))
    }

    pub async fn get(db: &Surreal<WsClient>, id: &str) -> surrealdb::Result<Option<Self>> {
        db.select(("job", id)).await
    }

    pub async fn update(db: &Surreal<WsClient>, id: &str, data: &Self) -> surrealdb::Result<Option<Self>> {
        db.update(("job", id)).content(data.clone()).await
    }

    pub async fn delete(db: &Surreal<WsClient>, id: &str) -> surrealdb::Result<()> {
        db.delete(("job", id)).await.map(|_: Option<Self>| ())
    }
}
