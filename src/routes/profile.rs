use axum::{response::IntoResponse, Extension, Json};

use crate::models::{dto::Profile, Error, User};

pub async fn get_profile_handler(
    Extension(user): Extension<User>,
) -> Result<impl IntoResponse, Error> {
    Ok(Json(Profile::from_user(&user)))
}
