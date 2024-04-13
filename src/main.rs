use axum::{response::{Html, IntoResponse}, serve, Router, routing::get};

async fn indexhtml() -> impl IntoResponse {
    let markup = tokio::fs::read("index.html").await.unwrap();

    return Html(markup);
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(indexhtml));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();
    serve(listener, app).await.unwrap();
}
