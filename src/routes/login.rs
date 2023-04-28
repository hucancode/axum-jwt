use argon2::{Argon2, PasswordHash, PasswordVerifier};
use axum::{
    extract::State,
    http::{Response, StatusCode},
    response::IntoResponse,
    Json,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde_json::{json, Value};
use std::sync::Arc;

use crate::{
    models::{dto::LoginInfo, TokenClaim, User},
    AppState,
};

pub async fn login_handler(
    State(state): State<Arc<AppState>>,
    Json(body): Json<LoginInfo>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let user = sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE email = $1",
        body.email.to_ascii_lowercase()
    )
    .fetch_one(&state.db)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "message": format!("Database error: {}", e),
            })),
        )
    })?;
    PasswordHash::new(&user.password)
        .and_then(|hash| Argon2::default().verify_password(body.password.as_bytes(), &hash))
        .map_err(|_| {
            (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "message": "Invalid password"
                })),
            )
        })?;
    let now = Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + Duration::minutes(60)).timestamp() as usize;
    let claims = TokenClaim {
        sub: user.id.to_string(),
        exp,
        iat,
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(state.env.jwt_secret.as_ref()),
    )
    .map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "message": "Error encoding JWT"
            })),
        )
    })
    .map(|token| Response::new(json!({ "token": token }).to_string()))
}
