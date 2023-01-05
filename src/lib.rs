extern crate reqwest;
extern crate serde;

pub mod internal;
pub mod models;
pub mod utils;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use internal::analytics::AnalyticsApi;
use internal::auth::AuthApi;
use internal::http_client::HttpClient;
use internal::posts::PostsApi;
use internal::user::UserApi;
use internal::users::UsersApi;
use utils::get_preview_url;

pub struct TabnewsClient {
    pub posts_api: PostsApi,
    pub analytics_api: AnalyticsApi,
    pub user_api: UserApi,
    pub users_api: UsersApi,
    pub auth_api: AuthApi,
    pub http_client: Rc<RefCell<HttpClient>>,
}

impl Default for TabnewsClient {
    fn default() -> Self {
        let client = Rc::new(RefCell::new(HttpClient::default()));

        TabnewsClient {
            http_client: Rc::clone(&client),
            posts_api: PostsApi::new(Rc::clone(&client)),
            analytics_api: AnalyticsApi::new(Rc::clone(&client)),
            user_api: UserApi::new(Rc::clone(&client)),
            users_api: UsersApi::new(Rc::clone(&client)),
            auth_api: AuthApi::new(Rc::clone(&client)),
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
            http_client: Rc::clone(&client),
            posts_api: PostsApi::new(Rc::clone(&client)),
            analytics_api: AnalyticsApi::new(Rc::clone(&client)),
            user_api: UserApi::new(Rc::clone(&client)),
            users_api: UsersApi::new(Rc::clone(&client)),
            auth_api: AuthApi::new(Rc::clone(&client)),
        }
    }

    pub async fn use_preview_url(self) -> Self {
        let preview_url = get_preview_url().await;

        {
            let mut client = self.http_client.borrow_mut();

            client.set_host(format!("{}/api/v1", preview_url));
        }

        self
    }
}
