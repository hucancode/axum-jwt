use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct TravelPass {
    pub id: Option<String>,
    pub route_id: String,
    pub card_id: String,
    pub expiry_date: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct TravelCard {
    pub id: Option<String>,
    pub balance: f64,
    pub last_stop_id: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct PassCardRelation {
    pub id: Option<String>,
    pub route_id: String,
    pub expiry_date: DateTime<Utc>,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct TravelCardWithPasses {
    pub id: Option<String>,
    pub balance: f64,
    pub last_stop_id: Option<String>,
    pub passes: Vec<PassCardRelation>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
