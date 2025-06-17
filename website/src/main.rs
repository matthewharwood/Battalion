use std::path::Path;
use std::sync::Arc;
use axum::routing::get;
use axum::Router;
use surrealdb::engine::remote::ws::{Client as WsClient, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;
use tera::Tera;
use tower_http::services::ServeDir;

use event::routes as event_routes;
use job::routes as job_routes;
use applicant::routes as applicant_routes;

pub struct AppState {
    pub views: Arc<Tera>,
    pub db: Arc<Surreal<WsClient>>,
}

#[tokio::main]
async fn main() {
    let public_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("public");
    println!("Public Dir: {:?}", public_dir);
    let static_files_service = ServeDir::new(public_dir).append_index_html_on_directories(false);
    let db = match Surreal::new::<Ws>("127.0.0.1:8000").await {
        Ok(s) => {
            println!("Surreal instance created");
            s
        }
        Err(e) => {
            panic!("Surreal initialization error: {}", e);
        }
    };
    
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
    
    let app  = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .merge(event_routes::router())
        .merge(job_routes::router())
        .merge(applicant_routes::router())
        .fallback_service(static_files_service);
    println!("Here in port 6969");
    let listener = tokio::net::TcpListener::bind("127.0.0.1:6969").await.unwrap();
    axum::serve(listener, app).await.unwrap();

}

fn views() -> Arc<Tera> {
    let mut tera = Tera::default();
    tera.add_template_files(vec![
        ("src/views/index.html", Some("index.html")),
        ("src/views/base.html", Some("base.html")),
        ("src/views/macros/forms.html", Some("macros/forms.html")),
        ("../event/templates/event_form.html", Some("event_form.html")),
        ("../event/templates/event_list.html", Some("event_list.html")),
        ("../job/templates/job_form.html", Some("job_form.html")),
        ("../job/templates/job_list.html", Some("job_list.html")),
        ("../applicant/templates/applicant_form.html", Some("applicant_form.html")),
        ("../applicant/templates/applicant_list.html", Some("applicant_list.html")),
    ]).expect("Failed to load templates");
    Arc::new(tera)
}

