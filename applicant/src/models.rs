use serde::{Deserialize, Serialize};
use surrealdb::{Surreal, engine::remote::ws::Client as WsClient, sql::Thing};
use url::Url;
use shared::impl_id_to_string_for;
use serde_with::{serde_as, FromInto};
use serde_withs::ThingString;

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Apply {
    pub id: Option<Thing>,
    #[serde_as(as = "Option<FromInto<ThingString>>")]
    pub event: Option<Thing>,
    #[serde_as(as = "Option<FromInto<ThingString>>")]
    pub job: Option<Thing>,
    pub name: String,
    pub github: Option<Url>,
    pub email: String,
    pub resume: Option<Url>,
    pub code_base: Option<String>,
    pub linkedin: Option<Url>,
    pub language: Option<String>,
    pub portfolio: Option<Url>,
    pub why_programming: Option<String>,
    pub ultimate_project: Option<String>,
    pub proud_work: Option<String>,
    pub future_skills: Option<String>,
    pub oncall_stories: Option<String>,
    pub focus_strategies: Option<String>,
    pub support_systems: Option<String>,
    pub comfort_food: Option<String>,
    pub weekend: Option<String>,
    pub travel_wish: Option<String>,
    pub truth1: Option<String>,
    pub truth2: Option<String>,
    pub lie: Option<String>,
}
impl_id_to_string_for!(Apply);

impl Apply {
    pub async fn create(self, db: &Surreal<WsClient>) -> surrealdb::Result<Self> {
        let created: Option<Self> = db.create("apply").content(self).await?;
        Ok(created.expect("create returned none"))
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

