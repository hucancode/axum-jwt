use serde::Serialize;
#[derive(Debug, Serialize)]
pub struct Message {
    pub message: String,
}
impl Message {
    pub fn new(msg: &str) -> Self {
        Self {
            message: msg.to_string(),
        }
    }
}
