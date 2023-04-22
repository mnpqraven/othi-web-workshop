use axum::{
    body::Body,
    http::{Request, StatusCode},
    response::Response,
    routing::get,
    Router,
};
use logging::tracing_test;

mod logging;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // examples on using the tracing crate
    tracing_test().await.unwrap();

    // build our application with a route
    let app = Router::new()
        .route("/", get(root))
        .route("/hyper", get(get_hyper_root).post(post_hyper_root));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Hello world"
}

async fn get_hyper_root(_req: Request<Body>) -> Result<Response<Body>, StatusCode> {
    let res = Response::new("Hello from hyper".into());
    Ok(res)
}
async fn post_hyper_root(_req: Request<Body>) -> Result<Response<Body>, StatusCode> {
    Err(StatusCode::BAD_REQUEST)
}
