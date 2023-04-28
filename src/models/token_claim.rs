use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaim {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}
