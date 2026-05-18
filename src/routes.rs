use axum::{extract::State, response::Json};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

pub mod check_conflicts {
    use super::*;

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Input {
        station_graph: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Output {
        success: bool,
    }

    pub async fn handler(
        State(state): State<Arc<Mutex<u64>>>,
        Json(_event): Json<Input>,
    ) -> Json<Output> {
        let mut point = state
            .lock()
            .expect("unlocking mutex should not return an error");
        *point += 1;
        Json(Output { success: true })
    }
}
