use axum::{response::IntoResponse, Extension, Json};

use crate::models::{dto::Profile, User};

pub async fn get_profile_handler(Extension(user): Extension<User>) -> impl IntoResponse {
    Json(Profile::from_user(&user))
}
