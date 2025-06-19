use std::path::Path;
use std::sync::Arc;
use axum::Router;
use axum::routing::get;
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;
use tera::Tera;
use tower_http::services::ServeDir;
use applicant::{self, AppState};
use event;
use job;
use review;
use vote;
use shared;

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
        .merge(applicant::routes::routes())
        .merge(event::routes::routes())
        .merge(job::routes::routes())
        .merge(review::routes::routes())
        .merge(vote::routes::routes())
        .fallback_service(static_files_service)
        .with_state(app_state);
    println!("Here in port 6969");
    let listener = tokio::net::TcpListener::bind("127.0.0.1:6969").await.unwrap();
    axum::serve(listener, app).await.unwrap();

}

fn views() -> Arc<Tera> {
    let mut tera = Tera::default();
    shared::add_templates(&mut tera);
    tera.add_template_files(vec![
        ("./applicant/templates/applicant_form.html", Some("applicant_form.html")),
        ("./applicant/templates/applicant_list.html", Some("applicant_list.html")),
        ("./event/templates/event_form.html", Some("event_form.html")),
        ("./event/templates/event_list.html", Some("event_list.html")),
        ("./job/templates/job_form.html", Some("job_form.html")),
        ("./job/templates/job_list.html", Some("job_list.html")),
        ("./review/templates/grid.html", Some("grid.html")),
        ("./vote/templates/vote_widget.html", Some("vote_widget.html")),
    ]).expect("Failed to load templates");
    Arc::new(tera)
}

