use argon2::{Argon2, PasswordHash, PasswordVerifier};
use axum::{extract::State, http::StatusCode, response::IntoResponse, response::Response, Json};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use std::sync::Arc;

use crate::{
    models::{dto::LoginInfo, Error, TokenClaim, User},
    AppState,
};

pub async fn login_handler(
    State(state): State<Arc<AppState>>,
    Json(body): Json<LoginInfo>,
) -> Result<impl IntoResponse, Error> {
    let user = state
        .db
        .select(("user", body.email.to_ascii_lowercase()))
        .await?;
    let user: User = user.ok_or((StatusCode::BAD_REQUEST, "User does not exist"))?;
    let hash = PasswordHash::new(&user.password)?;
    Argon2::default().verify_password(body.password.as_bytes(), &hash)?;
    let now = Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + Duration::days(7)).timestamp() as usize;
    let claims = TokenClaim {
        sub: user.email,
        exp,
        iat,
    };
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(state.config.jwt_secret.as_ref()),
    )?;

    Ok(Response::new(token))
}
