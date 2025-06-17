use std::sync::Arc;
use tera::Tera;
use surrealdb::{Surreal, engine::remote::ws::Client as WsClient};

pub struct AppState {
    pub views: Arc<Tera>,
    pub db: Arc<Surreal<WsClient>>,
}

pub mod models;
pub mod handlers;
pub mod routes;
