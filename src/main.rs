use axum::{response::{IntoResponse, Response}, routing::get, serve, Json, Router};

#[axum::debug_handler]
async fn indexhtml() -> Response<String> {
    let markup = tokio::fs::read_to_string("index.html").await.unwrap();

    return Response::builder()
        .status(200)
        .header("Content-Type", "text/html")
        .body(markup)
        .unwrap();
}

#[axum::debug_handler]
async fn indexcss() -> Response<String> {
    let markup = tokio::fs::read_to_string("index.css").await.unwrap();

    return Response::builder()
        .status(200)
        .header("Content-Type", "text/css")
        .body(markup)
        .unwrap();
}

#[axum::debug_handler]
async fn indexjs() -> Response<String> {
    let markup = tokio::fs::read_to_string("index.js").await.unwrap();

    return Response::builder()
        .status(200)
        .header("Content-Type", "text/javascript")
        .body(markup)
        .unwrap();
}

#[axum::debug_handler]
async fn cpuusage() -> impl IntoResponse {
    let mut sys = sysinfo::System::new();
    sys.refresh_cpu_usage();

    std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);

    sys.refresh_cpu_usage();

    let mut data: Vec<f32> = Vec::new();

    for cpu in sys.cpus() {
        data.push(cpu.cpu_usage());
    }

    return Json(data);
}

#[axum::debug_handler]
async fn sysinfo() -> impl IntoResponse {
    let mut data: Vec<String> = Vec::new();
    data.push(sysinfo::System::name().unwrap());
    data.push(sysinfo::System::kernel_version().unwrap());
    data.push(sysinfo::System::os_version().unwrap());
    data.push(sysinfo::System::host_name().unwrap());

    return Json(data);
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(indexhtml))
        .route("/index.css", get(indexcss))
        .route("/index.js", get(indexjs))
        .route("/api/cpu_usage", get(cpuusage))
        .route("/api/static_sysinfo", get(sysinfo));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000").await.unwrap();
    serve(listener, app).await.unwrap();
}
