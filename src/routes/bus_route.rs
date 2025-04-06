use crate::{
    models::{
        bus::BusRouteWithStops,
        dto::{bus_info::VehicleRouteRelationInfo, BusRouteAddStopsInfo, BusRouteAddVehiclesInfo, BusRouteCreateInfo, StopRouteRelationInfo},
        BusRoute, Error,
    },
    AppState,
};
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use std::iter;
use std::sync::Arc;

pub async fn get_all_handler(
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, Error> {
    let query = format!(
        "SELECT meta::id(id) AS id,
            name,
            created_at,
            updated_at,
            (SELECT meta::id(out.id) AS id, out.name AS name, index FROM ->contain) AS stops
        FROM bus_route;"
    );
    let routes: Vec<BusRoute> = state.db.query(query).await?.take(0)?;
    Ok(Json(routes))
}

pub async fn get_handler(
    Path(id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, Error> {
    let query = format!(
        "SELECT meta::id(id) AS id,
            name,
            created_at,
            updated_at,
            (SELECT meta::id(out.id) AS id, out.name AS name, index FROM ->contain) AS stops
        FROM ONLY bus_route:{id};"
    );
    let route: Option<BusRouteWithStops> = state.db.query(query).await?.take(0)?;
    Ok(Json(route))
}

pub async fn create_handler(
    State(state): State<Arc<AppState>>,
    Json(body): Json<BusRouteCreateInfo>,
) -> Result<impl IntoResponse, Error> {
    let create_query = format!(
        "let $route = CREATE ONLY bus_route SET name = '{}', created_at = time::now(), updated_at = time::now()",
        body.name
    );
    let relations: Vec<_> = iter::once(create_query)
        .chain(body.stops.iter().enumerate().map(|(i, name)| format!(
            "RELATE ONLY $route ->contain-> (CREATE ONLY bus_stop SET name = '{name}', update_at = time::now(), created_at = time::now()) SET order = {i}"
        )))
        .collect();
    state.db.query(relations.join(";")).await?;
    Ok(Json(body.name))
}

pub async fn add_stop_handler(
    State(state): State<Arc<AppState>>,
    Path(route_id): Path<String>,
    Json(body): Json<BusRouteAddStopsInfo>,
) -> Result<impl IntoResponse, Error> {
    let queries: Vec<_> = body.stop_ids.into_iter()
        .map(|StopRouteRelationInfo {id, order}| format!("RELATE bus_route:{route_id} ->contain bus_stop:{id} SET order = {order};"))
        .collect();
    state.db.query(queries.join(";")).await?;
    Ok(Json(route_id))
}

pub async fn add_vehicle_handler(
    State(state): State<Arc<AppState>>,
    Path(route_id): Path<String>,
    Json(body): Json<BusRouteAddVehiclesInfo>,
) -> Result<impl IntoResponse, Error> {
    let queries: Vec<_> = body.vehicle_ids.into_iter()
        .map(|VehicleRouteRelationInfo {id}| format!("RELATE bus_route:{route_id} ->contain vehicle:{id}"))
        .collect();
    state.db.query(queries.join(";")).await?;
    Ok(Json(route_id))
}
