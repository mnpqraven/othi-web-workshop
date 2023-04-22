use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware,
    response::Response,
    routing::get,
    Router,
};
use error::error_example;
use logging::{layer::print_request_response, logging_example};
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt};

mod error;
mod logging;

#[tokio::main]
async fn main() {
    // fmt() defaults to a INFO level tracing, if you want DEBUG you need to
    // specify it explicitly.

    // Simple init
    // tracing_subscriber::fmt()
    //     .with_max_level(Level::DEBUG)
    //     .init();
    // More detailed
    // https://github.com/tokio-rs/axum/blob/v0.6.x/examples/print-request-response/src/main.rs
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                "othi_web_workshop=debug,tower_http=debug,axum::rejection=trace".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // build our application with a route
    let app = Router::new()
        .route("/", get(root))
        // example of response using axum and converting errors into server
        // responses
        .route("/error", get(error_example))
        .route("/logging", get(logging_example))
        // example of basic response using hyper
        .route("/hyper", get(get_hyper_root).post(post_hyper_root))
        // custom tracing layer, go to logging/layer module for more details
        .layer(middleware::from_fn(print_request_response));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

/// For axum, the response can be anything that implements `IntoResponse`.
///
/// The error needs to implement `Infallible`, by default you can use
/// `StatusCode` for a specific status code response, but if you want to use
/// your custom error then you need to implement that Error `IntoResponse`
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
