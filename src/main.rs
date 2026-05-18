use systemlab21::routes::check_conflicts;
use systemlab21::utils;

use axum::{
    Router,
    http::StatusCode,
    routing::{get, post},
};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    utils::init_tracing();

    let app_state = Arc::new(Mutex::new(0));

    let app = Router::new()
        .route("/status", get(|| async { "this is an experiment" }))
        .route("/check_conflicts", post(check_conflicts::handler))
        .with_state(app_state)
        .layer((
            TraceLayer::new_for_http(),
            TimeoutLayer::with_status_code(StatusCode::REQUEST_TIMEOUT, Duration::from_secs(10)),
        ));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());

    let _ = axum::serve(listener, app).await;
}

