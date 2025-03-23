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
