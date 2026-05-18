use systemlab21::tracing::init_tracing;

use axum::{
    Router,
    extract::State,
    http::StatusCode,
    response::Json,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    init_tracing();

    let app_state = Arc::new(Mutex::new(0));

    let app = Router::new()
        .route("/status", get(|| async { "this is an experiment" }))
        .route("/check_conflicts", post(handle))
        .with_state(app_state)
        .layer((
            TraceLayer::new_for_http(),
            TimeoutLayer::with_status_code(StatusCode::REQUEST_TIMEOUT, Duration::from_secs(10)),
        ));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await;
}

#[derive(Debug, Serialize, Deserialize)]
struct Input {
    msg: String,
}

async fn handle(State(state): State<Arc<Mutex<u64>>>, Json(_event): Json<Input>) -> Json<Value> {
    let mut point = state
        .lock()
        .expect("unlocking mutex should not return an error");
    *point += 1;
    Json(json!({"status": "Ok"}))
}
