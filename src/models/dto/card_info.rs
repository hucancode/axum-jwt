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

// New relation DTOs
#[derive(Debug, Deserialize, Serialize)]
pub struct CardPassRelationInfo {
    pub id: String,
    pub pass_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CardAddPassesInfo {
    pub id: String,
    pub pass_ids: Vec<CardPassRelationInfo>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ValidationResult {
    pub valid: bool,
    pub deducted: f64,
    pub reason: String,
    pub timestamp: Option<DateTime<Utc>>,
    pub balance: Option<f64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PassRouteRelationInfo {
    pub id: String,
    pub card_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RouteAddPassesInfo {
    pub id: String,
    pub pass_ids: Vec<PassRouteRelationInfo>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TravelHistory {
    pub card_id: String,
    pub stop_id: String,
    pub timestamp: DateTime<Utc>,
    pub fare_deducted: Option<f64>,
    pub pass_used: Option<String>,
}
