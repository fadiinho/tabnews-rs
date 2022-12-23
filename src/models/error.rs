use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TabnewsError {
    pub name: String,
    pub message: String,
    pub action: String,
    pub status_code: u64,
    pub error_id: String,
    pub request_id: String,
    pub error_location_code: String,
}
