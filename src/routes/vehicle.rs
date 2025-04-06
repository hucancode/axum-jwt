use crate::{
    models::{dto::{VehicleCreateInfo, VehicleUpdateLocationInfo}, Error, Vehicle},
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
            plate_number,
            created_at,
            updated_at
        FROM vehicle;"
    );
    let stops: Vec<Vehicle> = state.db.query(query).await?.take(0)?;
    Ok(Json(stops))
}

pub async fn get_handler(
    Path(id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, Error> {
    let query = format!(
        "SELECT meta::id(id) AS id,
            plate_number,
            longitude,
            latitude,
            created_at,
            updated_at
        FROM ONLY vehicle:{id};"
    );
    let stop: Option<Vehicle> = state.db.query(query).await?.take(0)?;
    Ok(Json(stop))
}

pub async fn create_handler(
    State(state): State<Arc<AppState>>,
    Json(VehicleCreateInfo{plate_number, route}): Json<VehicleCreateInfo>,
) -> Result<impl IntoResponse, Error> {
    let mut queries = vec![format!(
        "LET $vehicle = CREATE ONLY vehicle
        SET plate_number = '{plate_number}',
            created_at = time::now(),
            updated_at = time::now()"
        )];
    if let Some(route) = route {
        queries.push(format!("RELATE ONLY route:{route} ->contain $vehicle"));
    }
    let result: Option<RecordId> = state.db.query(queries.join(";")).await?.take(0)?;
    Ok(Json(result))
}

pub async fn update_location_handler(
    Path(id): Path<String>,
    State(state): State<Arc<AppState>>,
    Json(VehicleUpdateLocationInfo{longitude, latitude}): Json<VehicleUpdateLocationInfo>,
) -> Result<impl IntoResponse, Error> {
    let query = format!(
        "UPDATE ONLY vehicle:{id}
        SET longitude = {longitude},
        latitude = {latitude},
        updated_at = time::now()"
    );
    // calculate nearest bus stop based on vehicle coordinate
    let result: Option<RecordId> = state.db.query(query).await?.take(0)?;
    Ok(Json(result))
}
