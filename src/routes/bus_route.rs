use crate::{
    models::{dto::BusRouteCreateInfo, BusRoute, Error},
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
    let routes: Vec<BusRoute> = state.db.select("bus_route").await?;
    Ok(Json(routes))
}

pub async fn get_handler(
    Path(id): Path<RecordIdKey>,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, Error> {
    let route: Option<BusRoute> = state.db.select(("bus_route", id)).await?;
    //let stops: Vec<BusStop> = state.db.select(("bus_stop", route.unwrap().stops)).await?;
    Ok(Json(route))
}

pub async fn create_handler(
    State(state): State<Arc<AppState>>,
    Json(body): Json<BusRouteCreateInfo>,
) -> Result<impl IntoResponse, Error> {
    let bus_route = BusRoute {
        name: body.name,
        stops: body.stops,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };
    let result: Option<BusRoute> = state.db.create("bus_route").content(bus_route).await?;
    Ok(Json(result))
}
