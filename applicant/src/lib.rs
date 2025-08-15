use std::sync::Arc;
use tera::Tera;
use surrealdb::{Surreal, engine::remote::ws::Client as WsClient};
use tokio::sync::broadcast;

pub struct AppState {
    pub views: Arc<Tera>,
    pub db: Arc<Surreal<WsClient>>, 
    pub broadcaster: broadcast::Sender<String>,
}

pub mod models;
pub mod handlers;
pub mod routes;

pub use handlers::*;