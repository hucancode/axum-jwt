use std::sync::Arc;

use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHasher,
};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::{
    models::{
        dto::{Message, Profile, RegisterInfo},
        User,
    },
    AppState,
};

pub async fn register_user_handler(
    State(state): State<Arc<AppState>>,
    Json(body): Json<RegisterInfo>,
) -> Result<impl IntoResponse, (StatusCode, Json<Message>)> {
    let exists = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM users WHERE email = $1)")
        .bind(body.email.to_owned().to_ascii_lowercase())
        .fetch_one(&state.db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(Message {
                    message: format!("Database error: {e}"),
                }),
            )
        })?;

    if exists {
        return Err((
            StatusCode::CONFLICT,
            Json(Message::new("User with that email already exists")),
        ));
    }

    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = Argon2::default()
        .hash_password(body.password.as_bytes(), &salt)
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(Message {
                    message: format!("Error while hashing password: {e}"),
                }),
            )
        })
        .map(|hash| hash.to_string())?;

    let user = sqlx::query_as!(
        User,
        "INSERT INTO users (name,email,password) VALUES ($1, $2, $3) RETURNING *",
        body.name.to_string(),
        body.email.to_string().to_ascii_lowercase(),
        hashed_password
    )
    .fetch_one(&state.db)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(Message {
                message: format!("Database error: {e}"),
            }),
        )
    })?;

    Ok(Json(Profile::from_user(&user)))
}
