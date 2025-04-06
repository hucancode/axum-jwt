use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct BusRoute {
    pub id: Option<String>,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct BusStop {
    pub id: Option<String>,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct StopRouteRelation {
    pub id: String,
    pub name: String,
    pub order: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BusRouteWithStops {
    pub id: Option<String>,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub stops: Vec<StopRouteRelation>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Vehicle {
    pub id: Option<String>,
    pub longitute: Option<f64>,
    pub latitude: Option<f64>,
    pub route_id: String,
    pub next_stop_id: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
