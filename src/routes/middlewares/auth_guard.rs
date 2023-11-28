use std::sync::Arc;

use axum::{
    body::Body,
    extract::State,
    http::{header, Request, StatusCode},
    middleware::Next,
    response::IntoResponse,
};

use jsonwebtoken::{decode, DecodingKey, Validation};

use crate::{
    app_state::AppState,
    models::{Error, TokenClaim, User},
};

pub async fn auth_guard(
    State(state): State<Arc<AppState>>,
    mut req: Request<Body>,
    next: Next,
) -> Result<impl IntoResponse, Error> {
    let token = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .and_then(|header| header.strip_prefix("Bearer "))
        .ok_or((
            StatusCode::UNAUTHORIZED,
            "You are not logged in, please provide token",
        ))?;
    let token = decode::<TokenClaim>(
        token,
        &DecodingKey::from_secret(state.config.jwt_secret.as_ref()),
        &Validation::default(),
    )?;
    let user: Option<User> = state.db.select(("user", token.claims.sub)).await?;
    let user = user.ok_or((StatusCode::UNAUTHORIZED, "No user match this token"))?;
    req.extensions_mut().insert(user);
    Ok(next.run(req).await)
}
