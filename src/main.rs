use axum::{response::Response, routing::get, serve, Router};

#[axum::debug_handler]
async fn indexhtml() -> Response<String> {
    let markup = include_str!("index.html").to_string();

    return Response::builder()
        .status(200)
        .header("Content-Type", "text/html")
        .body(markup)
        .unwrap();
}

#[axum::debug_handler]
async fn indexcss() -> Response<String> {
    let markup = include_str!("index.css").to_string();

    return Response::builder()
        .status(200)
        .header("Content-Type", "text/css")
        .body(markup)
        .unwrap();
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(indexhtml))
        .route("/index.css", get(indexcss));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();
    serve(listener, app).await.unwrap();
}
