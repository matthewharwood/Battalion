use std::sync::Arc;
use surrealdb::{Surreal, engine::remote::ws::Client as WsClient};
use tera::Tera;

#[derive(Clone)]
pub struct AppState {
    pub views: Arc<Tera>,
    pub db: Arc<Surreal<WsClient>>,
}
