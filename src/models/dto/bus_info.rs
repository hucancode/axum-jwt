use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct BusRouteCreateInfo {
    pub name: String,
    pub stops: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct BusStopCreateInfo {
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct StopRouteRelationInfo {
    pub id: String,
    pub order: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BusRouteAddStopsInfo {
    pub id: String,
    pub stop_ids: Vec<StopRouteRelationInfo>,
}

#[derive(Debug, Deserialize)]
pub struct VehicleCreateInfo {
    pub plate_number: String,
    pub route: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct VehicleUpdateLocationInfo {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VehicleRouteRelationInfo {
    pub id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BusRouteAddVehiclesInfo {
    pub id: String,
    pub vehicle_ids: Vec<VehicleRouteRelationInfo>,
}
