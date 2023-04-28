use axum::{http::StatusCode, response::IntoResponse, Extension, Json};
use serde_json::{json, Value};

use crate::models::{dto::Profile, User};

pub async fn get_profile_handler(
    Extension(user): Extension<User>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let json_response = json!({
        "status":  "success",
        "data": json!({
            "user": Profile::from_user(&user)
        })
    });

    Ok(Json(json_response))
}
