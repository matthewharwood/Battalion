use axum::{response::Html, extract::Path};

pub async fn list() -> Html<&'static str> {
    Html("<h1>stub</h1>")
}

pub async fn new_form() -> Html<&'static str> {
    Html("<h1>stub</h1>")
}

pub async fn create() -> Html<&'static str> {
    Html("<h1>stub</h1>")
}

pub async fn edit(Path(_id): Path<String>) -> Html<&'static str> {
    Html("<h1>stub</h1>")
}

pub async fn delete(Path(_id): Path<String>) -> Html<&'static str> {
    Html("<h1>stub</h1>")
}
