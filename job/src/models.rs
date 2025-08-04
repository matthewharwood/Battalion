use serde::{Deserialize, Serialize};
use surrealdb::{Surreal, engine::remote::ws::Client as WsClient, sql::Thing};
use shared::impl_id_to_string_for;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum JobLevel {
    Intern,
    #[serde(alias = "junior")]
    Junior,
    #[serde(alias = "mid")]
    Mid,
    #[serde(alias = "senior")]
    Senior,
    #[serde(alias = "staff")]
    Staff,
}


#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Job {
    pub id: Option<Thing>,
    pub value: Option<String>,
    pub title: String,
    pub level: JobLevel,
    pub team: String,
    pub location: String,
    pub description: String,
}

impl_id_to_string_for!(Job);

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
