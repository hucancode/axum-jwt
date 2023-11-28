use chrono::Utc;
use std::sync::Arc;
use axum::{extract::{State, Path}, response::IntoResponse, Json};
use crate::{
    models::{
        dto::{Profile, RegisterInfo},
        Error, User,
    },
    AppState,
};
use surrealdb::sql::Uuid;

pub async fn get_all_handler(
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, Error> {
    let products: Json = state
        .db
        .select("product")
        .await?;
    Ok(products)
}
pub async fn get_handler(
    Path(id): Path<Uuid>, 
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, Error> {
    let product = state
        .db
        .select(("product", id))
        .await?;
    Ok(Json(product))
}

pub async fn post_handler(Extension(user): Extension<User>) -> impl IntoResponse {

}

pub async fn get_variant_handler(Extension(user): Extension<User>) -> impl IntoResponse {

}
