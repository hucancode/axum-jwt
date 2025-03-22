use serde::Deserialize;
use surrealdb::RecordIdKey;

#[derive(Debug, Deserialize)]
pub struct BusRouteCreateInfo {
    pub name: String,
    pub stops: Vec<RecordIdKey>,
}

#[derive(Debug, Deserialize)]
pub struct BusStopCreateInfo {
    pub name: String,
}
