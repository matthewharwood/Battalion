use std::{path::Path, sync::Arc};

use axum::{routing::get, Router, response::Html};
use surrealdb::{Surreal, engine::remote::ws::{Ws, Client as WsClient}, opt::auth::Root};
use tera::{Tera, Context};
use tower_http::services::ServeDir;

use event::routes as event_routes;
use job::routes as job_routes;
use applicant::routes as applicant_routes;

pub struct AppState {
    pub views: Arc<Tera>,
    pub db: Arc<Surreal<WsClient>>,
}

// Handler for form submission
// async fn submit_apply(
//     State(state): State<Arc<AppState>>,
//     Form(apply): Form<Apply>,
// ) -> Redirect {
//     println!("Handler: Received form submission ---------------->");
//     let _ = apply.create(&state.db).await;
//     Redirect::to("/home")
// }

#[tokio::main]
async fn main() {
    let public_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("public");
    println!("Public Dir: {:?}", public_dir);
    // let static_files_service = ServeDir::new(&public_dir).append_index_html_on_directories(false); // We are not using this file

    let db = Surreal::new::<Ws>("127.0.0.1:8000").await.expect("Failed to connect to SurrealDB");
    db.signin(Root { username: "root", password: "root" })
        .await.expect("Failed to sign in");
    db.use_ns("test").use_db("test").await.expect("Failed to use DB");

    println!("Surreal instance created");

    let arc_views = load_views();

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/home", {
            let views = arc_views.clone();
            get(move || {
                let views = views.clone();
                async move {
                    let ctx = Context::new();
                    let html = views.render("index.html", &ctx)
                        .unwrap_or_else(|e| format!("Template error: {e}"));
                    Html(html)
                }
            })
        })
        .merge(event_routes::router())
        .merge(job_routes::router())
        .merge(applicant_routes::router())
        .fallback_service(ServeDir::new(&public_dir));

    println!("Here in port 6969");
    let listener = tokio::net::TcpListener::bind("127.0.0.1:6969").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn load_views() -> Arc<Tera> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    let mut tera = Tera::default();
    tera.add_template_files(vec![
        (root.join("website/src/views/index.html"), Some("index.html")),
        (root.join("website/src/views/base.html"), Some("base.html")),
        (root.join("website/src/views/macros/forms.html"), Some("macros/forms.html")),
        (root.join("event/templates/event_form.html"), Some("event_form.html")),
        (root.join("event/templates/event_list.html"), Some("event_list.html")),
        (root.join("job/templates/job_form.html"), Some("job_form.html")),
        (root.join("job/templates/job_list.html"), Some("job_list.html")),
        (root.join("applicant/templates/applicant_form.html"), Some("applicant_form.html")),
        (root.join("applicant/templates/applicant_list.html"), Some("applicant_list.html")),
    ]).expect("Failed to load templates");
    Arc::new(tera)
}

