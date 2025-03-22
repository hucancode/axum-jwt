use crate::{
    models::{dto::BusStopCreateInfo, BusStop, Error},
    AppState,
};
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use chrono::Utc;
use std::sync::Arc;
use surrealdb::RecordIdKey;

pub async fn get_all_handler(
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, Error> {
    let stops: Vec<BusStop> = state.db.select("bus_stop").await?;
    Ok(Json(stops))
}

pub async fn get_handler(
    Path(id): Path<RecordIdKey>,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, Error> {
    let stop: Option<BusStop> = state.db.select(("bus_stop", id)).await?;
    Ok(Json(stop))
}

pub async fn create_handler(
    State(state): State<Arc<AppState>>,
    Json(body): Json<BusStopCreateInfo>,
) -> Result<impl IntoResponse, Error> {
    let bus_stop = BusStop {
        name: body.name,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };
    let result: Option<BusStop> = state.db.create("bus_stop").content(bus_stop).await?;
    Ok(Json(result))
}
