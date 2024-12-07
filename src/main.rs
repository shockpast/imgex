use std::env;

use tower_http::{
    services::{ServeDir, ServeFile},
    trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer},
};
use tracing::Level;

mod routes;
mod utils;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    dotenvy::dotenv().unwrap();

    let router = axum::Router::new()
        .merge(routes::api::serve())
        .nest_service(
            "/",
            ServeDir::new("data").not_found_service(ServeFile::new("public/index.html")),
        )
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_response(
                    DefaultOnResponse::new()
                        .level(Level::INFO)
                        .include_headers(true)
                        .latency_unit(tower_http::LatencyUnit::Millis),
                ),
        );

    let listener = tokio::net::TcpListener::bind(format!(
        "0.0.0.0:{}",
        env::var("SERVER_PORT").unwrap().as_str()
    ))
    .await
    .unwrap();

    axum::serve(listener, router).await.unwrap();
}
