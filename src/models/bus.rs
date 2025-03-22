use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::RecordIdKey;

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct BusRoute {
    pub name: String,
    pub stops: Vec<RecordIdKey>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct BusRouteFat {
    pub name: String,
    pub stops: Vec<BusStop>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct BusStop {
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
