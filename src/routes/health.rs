use crate::models::dto::Message;
use axum::{response::IntoResponse, Json};

pub async fn health_checker_handler() -> impl IntoResponse {
    Json(Message::new(
        "JWT Authentication in Rust using Axum, SurrealDB",
    ))
}
