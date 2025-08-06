use serde::{Deserialize, Serialize};
use surrealdb::{Surreal, engine::remote::ws::Client as WsClient, sql::Thing};
use shared::impl_id_to_string_for;
use serde_with::{serde_as, FromInto};
use serde_withs::ThingString;

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Apply {
    pub id: Option<Thing>,
    #[serde_as(as = "Option<FromInto<ThingString>>")]
    pub event: Option<Thing>,
    #[serde_as(as = "Option<FromInto<ThingString>>")]
    pub job: Option<Thing>,
    pub name: String,
    pub github: Option<String>,
    pub email: String,
    pub resume: Option<String>,
    pub linkedin: Option<String>,
    pub portfolio: Option<String>,
    pub whatprogramming: Option<String>,
    pub whyprogramming: Option<String>,
    pub start: Option<String>,
    pub program: Option<String>,
    pub project: Option<String>,
    pub proudwork: Option<String>,
    pub futureskills: Option<String>,
    pub stories: Option<String>,
    pub strategies: Option<String>,
    pub support: Option<String>,
    pub food: Option<String>,
    pub weekend: Option<String>,
    pub travel: Option<String>,
}
impl_id_to_string_for!(Apply);

impl Apply {
    pub async fn create(self, db: &Surreal<WsClient>) -> surrealdb::Result<Self> {
        let created: Option<Self> = db.create("apply").content(self).await?;
        let result = created.expect("create returned none");
        
        // Get the ID and fetch the record again to ensure proper deserialization
        if let Some(id) = &result.id {
            let fetched: Option<Self> = db.select(("apply", id.id.to_string())).await?;
            if let Some(fetched_record) = fetched {
                println!("Fetched record from DB: {:?}", fetched_record);
                return Ok(fetched_record);
            }
        }
        
        Ok(result)
    }

    pub async fn get(db: &Surreal<WsClient>, id: &str) -> surrealdb::Result<Option<Self>> {
        db.select(("apply", id)).await
    }

    pub async fn update(db: &Surreal<WsClient>, id: &str, data: &Self) -> surrealdb::Result<Option<Self>> {
        db.update(("apply", id)).content(data.clone()).await
    }

    pub async fn delete(db: &Surreal<WsClient>, id: &str) -> surrealdb::Result<()> {
        db.delete(("apply", id)).await.map(|_: Option<Self>| ())
    }
}

