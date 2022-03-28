use serde::{Deserialize, Serialize};

// base info
#[derive(CRUDTable, Serialize, Deserialize, Clone, Debug)]
pub struct BaseResult {
    pub code: String,
    pub data: String,
    pub success: bool,
}