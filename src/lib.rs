extern crate reqwest;
extern crate serde;

pub mod models;
pub mod tabnews;

use std::collections::HashMap;

use tabnews::http_client::HttpClient;

use tabnews::user::UserApi;
use tabnews::{analytics::AnalyticsApi, posts::PostsApi};

pub struct TabnewsClient {
    pub posts_api: PostsApi,
    pub analytics_api: AnalyticsApi,
    pub user_api: UserApi,
}

impl Default for TabnewsClient {
    fn default() -> Self {
        let client = HttpClient::default();

        TabnewsClient {
            posts_api: PostsApi::new(client.clone()),
            analytics_api: AnalyticsApi::new(client.clone()),
            user_api: UserApi::new(client.clone()),
        }
    }
}

impl TabnewsClient {
    pub fn new(headers: HashMap<String, String>) -> Self {
        let mut client = HttpClient::default();

        client.add_multiple_headers(headers.clone());

        TabnewsClient {
            posts_api: PostsApi::new(client.clone()),
            analytics_api: AnalyticsApi::new(client.clone()),
            user_api: UserApi::new(client.clone()),
        }
    }
}
