use crate::models::analytics::{CommentsPublishedStatus, PostsPublishedStatus, UsersCreatedStatus};

use std::cell::RefCell;
use std::rc::Rc;

use super::http_client::HttpClient;

pub struct AnalyticsApi {
    http_client: Rc<RefCell<HttpClient>>,
}

impl Default for AnalyticsApi {
    fn default() -> Self {
        let http_client = Rc::new(RefCell::new(HttpClient::default()));

        AnalyticsApi::new(http_client)
    }
}

impl AnalyticsApi {
    pub fn new(client: Rc<RefCell<HttpClient>>) -> Self {
        AnalyticsApi {
            http_client: client,
        }
    }

    /// Get how many users have been created per day
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use tabnews::models::analytics::UsersCreatedStatus;
    /// # use tabnews::TabnewsClient;
    /// # #[tokio::main]
    /// # async fn main() {
    /// let client = TabnewsClient::default();
    ///
    /// let users_status: Vec<UsersCreatedStatus> = client.analytics_api
    ///     .get_users_created().await;
    ///
    /// assert!(!users_status.is_empty());
    /// # }
    /// ```
    pub async fn get_users_created(&self) -> Vec<UsersCreatedStatus> {
        let _client = self.http_client.borrow();

        let response = _client
            .get("/analytics/users-created".to_owned())
            .await
            .unwrap();

        let json_response: Vec<UsersCreatedStatus> = response.json().await.unwrap();

        json_response
    }

    /// Get how many posts have been created per day
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use tabnews::models::analytics::PostsPublishedStatus;
    /// # use tabnews::TabnewsClient;
    /// # #[tokio::main]
    /// # async fn main() {
    /// let client = TabnewsClient::default();
    /// let posts_status: Vec<PostsPublishedStatus> = client.analytics_api
    ///     .get_posts_published().await;
    ///
    /// assert!(!posts_status.is_empty());
    /// # }
    /// ```
    pub async fn get_posts_published(&self) -> Vec<PostsPublishedStatus> {
        let _client = self.http_client.borrow();

        let response = _client
            .get("/analytics/root-content-published".to_owned())
            .await
            .unwrap();

        let json_response: Vec<PostsPublishedStatus> = response.json().await.unwrap();

        json_response
    }

    /// Get how many comments have been created per day
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use tabnews::TabnewsClient;
    /// # use tabnews::models::analytics::CommentsPublishedStatus;
    /// # #[tokio::main]
    /// # async fn main() {
    /// let client = TabnewsClient::default();
    /// let comments_status: Vec<CommentsPublishedStatus> = client.analytics_api
    ///     .get_comments_published().await;
    ///
    /// assert!(!comments_status.is_empty());
    /// # }
    /// ```
    pub async fn get_comments_published(&self) -> Vec<CommentsPublishedStatus> {
        let _client = self.http_client.borrow();

        let response = _client
            .get("/analytics/child-content-published".to_owned())
            .await
            .unwrap();

        let json_response: Vec<CommentsPublishedStatus> = response.json().await.unwrap();

        json_response
    }
}
