use axum::response::Json;
use serde::{Deserialize, Serialize};

use crate::station::{Station, StationInput};

pub mod check_conflicts {
    use super::*;

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Input {
        station_graph: Vec<StationInput>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Output {
        success: bool,
    }

    pub async fn handler(Json(payload): Json<Input>) -> Json<Output> {
        let _station = Station::new(payload.station_graph);
        Json(Output { success: true })
    }
}
