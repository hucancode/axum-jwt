use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LoginInfo {
    pub email: String,
    pub password: String,
}
