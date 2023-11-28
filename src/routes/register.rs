use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHasher,
};
use axum::http::StatusCode;
use axum::{extract::State, response::IntoResponse, Json};
use chrono::Utc;
use std::sync::Arc;

use crate::{
    models::{
        dto::{Profile, RegisterInfo},
        Error, User,
    },
    AppState,
};

pub async fn register_user_handler(
    State(state): State<Arc<AppState>>,
    Json(body): Json<RegisterInfo>,
) -> Result<impl IntoResponse, Error> {
    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = Argon2::default()
        .hash_password(body.password.as_bytes(), &salt)?
        .to_string();
    let data = User {
        name: body.name,
        email: body.email.to_ascii_lowercase(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
        password: hashed_password,
        ..Default::default()
    };
    let user: Option<User> = state
        .db
        .create(("user", &data.email.to_ascii_lowercase()))
        .content(data)
        .await?;
    let user = user.ok_or((StatusCode::BAD_REQUEST, "Failed to create user"))?;
    Ok(Json(Profile::from(user)))
}
