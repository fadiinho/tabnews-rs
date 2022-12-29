extern crate reqwest;
extern crate serde;

pub mod internal;
pub mod models;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

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
        let client = Rc::new(RefCell::new(HttpClient::default()));

        TabnewsClient {
            posts_api: PostsApi::new(Rc::clone(&client)),
            analytics_api: AnalyticsApi::new(Rc::clone(&client)),
            user_api: UserApi::new(Rc::clone(&client)),
            users_api: UsersApi::new(Rc::clone(&client)),
        }
    }
}

impl TabnewsClient {
    pub fn new(headers: HashMap<String, String>) -> Self {
        let client = Rc::new(RefCell::new(HttpClient::default()));

        {
            let mut _client = client.borrow_mut();

            _client.add_multiple_headers(headers);
        }

        TabnewsClient {
            posts_api: PostsApi::new(Rc::clone(&client)),
            analytics_api: AnalyticsApi::new(Rc::clone(&client)),
            user_api: UserApi::new(Rc::clone(&client)),
            users_api: UsersApi::new(Rc::clone(&client)),
        }
    }
}
