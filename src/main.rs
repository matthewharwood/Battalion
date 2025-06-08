use std::sync::Arc;
use axum::response::{Html, IntoResponse};
use axum::{Form, Router};
use axum::extract::State;
use axum::routing::get;
use serde::{Deserialize, Serialize};
use surrealdb::engine::remote::ws::{Client as WsClient, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;
use tera::Tera;

struct AppState {
    pub views: Arc<Tera>,
    pub db: Arc<Surreal<WsClient>>,
}

#[tokio::main]
async fn main() {
    println!("1");
    let db = match Surreal::new::<Ws>("127.0.0.1:8000").await {
        Ok(s) => {
            println!("Surreal instance created");
            s
        }
        Err(e) => {
            panic!("Surreal initialization error: {}", e);
        }
    };
    println!("2");
    if let Err(e) = db
        .signin(Root {
            username: "root",
            password: "root",
        })
        .await
    {
        eprintln!("FATAL: Could not sign in to SurrealDB: {:?}", e);
        ::std::process::exit(1);
    };
    println!("3");
    if let Err(e) = db.use_ns("test").use_db("test").await {
        eprintln!(
            "FATAL: Could not use namespace/database in SurrealDB: {:?}",
            e
        );
        ::std::process::exit(1);
    }

    let shared_db = Arc::new(db);
    let arc_views = views();
    let app_state = Arc::new(AppState {
        views: arc_views,
        db: shared_db,
    });
    println!("4");
    let app  = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/apply", get(show_form).post(accept_form))
        .with_state(app_state);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:6969").await.unwrap();
    axum::serve(listener, app).await.unwrap();

}

fn views() -> Arc<Tera> {
    let tera_instance = match Tera::new("src/views/**/*.html") {
        Ok(t) => {
            println!("Tera instance created");
            t
        }
        Err(e) => {
            panic!("Tera initialization error: {}", e);
        }
    };
    Arc::new(tera_instance)

}

async fn show_form(State(app_state): State<Arc<AppState>>) -> impl IntoResponse {
    let tera = &app_state.views;
    let context = tera::Context::new();
    let rendered = tera.render("index.html", &context).unwrap();
    Html(rendered)
}

#[derive(Deserialize, Serialize, Debug)]
struct Input {
    name: String,
}


async fn accept_form(State(app_state): State<Arc<AppState>>, Form(input): Form<Input>) -> Html<String> {
    let db = &app_state.db;
    match db.insert::<Vec<Input>>("job").content(input).await {
        Ok(records) => {
            println!("Insert successful! Records returned: {:?}", records);
            println!("Number of records: {}", records.len());
            // Also try a query right after insert to verify
            let query_result: Result<Vec<Input>, _> = db.select("job").await;
            println!("Immediate query result: {:?}", query_result);

            if let Some(record) = records.first() {
                Html(format!("Successfully inserted job: {}", record.name))
            } else {
                Html(format!("Insert returned empty records vec"))
            }
        }
        Err(e) => {
            eprintln!("Insert failed: {:?}", e);
            Html(format!("Error: Failed to insert job - {}", e))
        }
    }
}
