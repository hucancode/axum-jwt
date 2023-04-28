use std::sync::Arc;

use axum::{
    extract::State,
    http::{header, Request, StatusCode},
    middleware::Next,
    response::IntoResponse,
    Json,
};

use jsonwebtoken::{decode, DecodingKey, Validation};
use serde_json::{json, Value};

use crate::{
    app_state::AppState,
    models::{TokenClaim, User},
};

pub async fn auth_guard<B>(
    State(data): State<Arc<AppState>>,
    mut req: Request<B>,
    next: Next<B>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let token = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .and_then(|header| header.strip_prefix("Bearer "))
        .ok_or((
            StatusCode::UNAUTHORIZED,
            Json(json!({
                "message": "You are not logged in, please provide token",
            })),
        ))?;
    let token = decode::<TokenClaim>(
        token,
        &DecodingKey::from_secret(data.env.jwt_secret.as_ref()),
        &Validation::default(),
    )
    .map_err(|_| {
        (
            StatusCode::UNAUTHORIZED,
            Json(json!({
                "message": "Invalid token",
            })),
        )
    })?;
    let user_id = uuid::Uuid::parse_str(&token.claims.sub).map_err(|_| {
        (
            StatusCode::UNAUTHORIZED,
            Json(json!({
                "message": "Invalid token",
            })),
        )
    })?;
    let user = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", user_id)
        .fetch_one(&data.db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "message": format!("Error fetching user from database: {}", e),
                })),
            )
        })?;
    req.extensions_mut().insert(user);
    Ok(next.run(req).await)
}
