use axum::response::Json;
use serde::{Deserialize, Serialize};

use crate::station::{Station, StationEdge, StationEdgeState};

pub mod check_conflicts {
    use super::*;

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Input {
        station_graph: Vec<StationEdge>,
        routes: Vec<StationEdgeState>,
        check_route: StationEdge,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Output {
        pub success: bool,
    }

    pub async fn handler(Json(payload): Json<Input>) -> Json<Output> {
        let mut station = Station::new(payload.station_graph);
        station.update_occupancy(payload.routes);
        let success = station.check_route(payload.check_route);
        Json(Output { success })
    }
}
