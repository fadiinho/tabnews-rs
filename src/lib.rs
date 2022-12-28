extern crate reqwest;
extern crate serde;

pub mod internal;
pub mod models;

use std::collections::HashMap;

use internal::analytics::AnalyticsApi;
use internal::http_client::HttpClient;
use internal::posts::PostsApi;
use internal::user::UserApi;
use internal::users::UsersApi;

pub struct TabnewsClient {
    pub posts_api: PostsApi,
    pub analytics_api: AnalyticsApi,
    pub user_api: UserApi,
    pub users_api: UsersApi,
}

impl Default for TabnewsClient {
    fn default() -> Self {
        let client = HttpClient::default();

        TabnewsClient {
            posts_api: PostsApi::new(client.clone()),
            analytics_api: AnalyticsApi::new(client.clone()),
            user_api: UserApi::new(client.clone()),
            users_api: UsersApi::new(client.clone()),
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
            users_api: UsersApi::new(client.clone()),
        }
    }
}
