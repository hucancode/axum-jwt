use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RegisterInfo {
    pub name: String,
    pub email: String,
    pub password: String,
}
