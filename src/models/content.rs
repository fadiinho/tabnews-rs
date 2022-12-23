use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Content {
    pub id: String,
    pub parent_id: Option<String>,
    pub owner_id: String,
    pub slug: String,
    pub body: Option<String>,
    pub title: Option<String>,
    pub status: String,
    pub source_url: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub published_at: String,
    pub deleted_at: Option<String>,
    pub tabcoins: u64,
    pub owner_username: String,
    pub children_deep_count: u64,
}

#[derive(Serialize)]
pub struct ContentParams {
    pub page: Option<u64>,
    pub per_page: Option<u64>,
    pub strategy: Option<String>,
}

impl Default for ContentParams {
    fn default() -> Self {
        ContentParams {
            page: None,
            per_page: None,
            strategy: None,
        }
    }
}
