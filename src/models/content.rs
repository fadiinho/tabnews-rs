use serde::{Deserialize, Serialize};

pub enum TabcoinsTransaction {
    Credit,
    Debit,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Tabcoins {
    pub tabcoins: i64,
}

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
    pub tabcoins: i64,
    pub owner_username: String,
    pub children_deep_count: u64,
    pub children: Option<Box<Vec<Content>>>,
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
