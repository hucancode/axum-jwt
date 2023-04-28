use chrono::prelude::*;
use serde::Serialize;
use crate::models::user::User;

#[allow(non_snake_case)]
#[derive(Debug, Serialize)]
pub struct Profile {
    pub id: String,
    pub name: String,
    pub email: String,
    pub role: String,
    pub photo: String,
    pub verified: bool,
    pub createdAt: DateTime<Utc>,
    pub updatedAt: DateTime<Utc>,
}

impl Profile {
    pub fn from_user(user: &User) -> Self {
        Self {
            id: user.id.to_string(),
            email: user.email.to_owned(),
            name: user.name.to_owned(),
            photo: user.photo.to_owned(),
            role: user.role.to_owned(),
            verified: user.verified,
            createdAt: user.created_at.unwrap(),
            updatedAt: user.updated_at.unwrap(),
        }
    }
}
