use axum::{http::StatusCode, response::IntoResponse, Extension, Json};

use crate::models::{
    dto::{Message, Profile},
    User,
};

pub async fn get_profile_handler(
    Extension(user): Extension<User>,
) -> Result<impl IntoResponse, (StatusCode, Json<Message>)> {
    Ok(Json(Profile::from_user(&user)))
}
