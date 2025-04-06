use crate::{
    models::{dto::BusStopCreateInfo, BusStop, Error},
    AppState,
};
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use std::sync::Arc;
use surrealdb::RecordId;

pub async fn get_all_handler(
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, Error> {
    let query = format!(
        "SELECT meta::id(id) AS id,
            name,
            created_at,
            updated_at
        FROM bus_stop"
    );
    let stops: Vec<BusStop> = state.db.query(query).await?.take(0)?;
    Ok(Json(stops))
}

pub async fn get_handler(
    Path(id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, Error> {
    let query = format!(
        "SELECT meta::id(id) AS id,
            name,
            created_at,
            updated_at
        FROM ONLY bus_stop:{id}"
    );
    let stop: Option<BusStop> = state.db.query(query).await?.take(0)?;
    Ok(Json(stop))
}

pub async fn create_handler(
    State(state): State<Arc<AppState>>,
    Json(BusStopCreateInfo { name }): Json<BusStopCreateInfo>,
) -> Result<impl IntoResponse, Error> {
    let query = format!(
        "CREATE ONLY bus_stop
        SET name = '{name}',
            created_at = time::now(),
            updated_at = time::now()"
    );
    let result: Option<RecordId> = state.db.query(query).await?.take(0)?;
    Ok(Json(result))
}
