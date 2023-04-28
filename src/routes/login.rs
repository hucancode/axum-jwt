use std::sync::Arc;

use argon2::{Argon2, PasswordHash, PasswordVerifier};
use axum::{
    extract::State,
    http::{ Response, StatusCode},
    response::IntoResponse,
    Json,
};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde_json::{json, Value};

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
    .fetch_optional(&state.db)
    .await
    .map_err(|e| {
        let error_response = json!({
            "status": "error",
            "message": format!("Database error: {}", e),
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?
    .ok_or_else(|| {
        let error_response = json!({
            "status": "fail",
            "message": "Invalid email or password",
        });
        (StatusCode::BAD_REQUEST, Json(error_response))
    })?;

    let is_valid = match PasswordHash::new(&user.password) {
        Ok(parsed_hash) => Argon2::default()
            .verify_password(body.password.as_bytes(), &parsed_hash)
            .map_or(false, |_| true),
        Err(_) => false,
    };

    if !is_valid {
        let error_response = json!({
            "status": "fail",
            "message": "Invalid email or password"
        });
        return Err((StatusCode::BAD_REQUEST, Json(error_response)));
    }

    let now = chrono::Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + chrono::Duration::minutes(60)).timestamp() as usize;
    let claims = TokenClaim {
        sub: user.id.to_string(),
        exp,
        iat,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(state.env.jwt_secret.as_ref()),
    )
    .unwrap();

    let response = Response::new(
        json!({
            "status": "success", 
            "token": token
        }).to_string());
    Ok(response)
}
