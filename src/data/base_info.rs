use serde::{Serialize, Deserialize};

// base info
#[derive(Serialize, Deserialize)]
pub struct BaseResult {
    pub code: String,
    pub data: String,
    pub success: bool,
}