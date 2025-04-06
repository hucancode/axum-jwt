use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CardCreateInfo {
    pub balance: f64,
}
#[derive(Debug, Deserialize)]
pub struct TravelCardRechargeInfo {
    pub amount: f64,
}

#[derive(Debug, Deserialize)]
pub struct TravelPassCreateInfo {
    pub card_id: String,
    pub route_id: String,
    pub expiry_date: DateTime<Utc>,
}
