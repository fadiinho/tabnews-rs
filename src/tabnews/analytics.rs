use crate::models::analytics::{CommentsPublishedStatus, PostsPublishedStatus, UsersCreatedStatus};

use super::client::TabnewsApi;

pub struct AnalyticsApi {
    tabnews_client: TabnewsApi,
}

impl AnalyticsApi {
    pub fn new() -> Self {
        let tabnews_client = TabnewsApi::default();

        AnalyticsApi { tabnews_client }
    }

    /// Get how many users have been created per day
    ///
    /// # Examples
    ///
    /// ```
    /// use tabnews_rs::tabnews::analytics::AnalyticsApi;
    /// use tabnews_rs::models::analytics::UsersCreatedStatus;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let analytics_api = AnalyticsApi::new();
    ///     let response: Vec<UsersCreatedStatus> = analytics_api
    ///         .get_users_created().await;
    ///
    ///     println!("{:?}", response)
    /// }
    /// ```
    pub async fn get_users_created(&self) -> Vec<UsersCreatedStatus> {
        let response = self
            .tabnews_client
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
    /// ```
    /// use tabnews_rs::tabnews::analytics::AnalyticsApi;
    /// use tabnews_rs::models::analytics::PostsPublishedStatus;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let analytics_api = AnalyticsApi::new();
    ///     let response: Vec<PostsPublishedStatus> = analytics_api
    ///         .get_posts_published().await;
    ///
    ///     println!("{:?}", response)
    /// }
    /// ```
    pub async fn get_posts_published(&self) -> Vec<PostsPublishedStatus> {
        let response = self
            .tabnews_client
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
    /// ```
    /// use tabnews_rs::tabnews::analytics::AnalyticsApi;
    /// use tabnews_rs::models::analytics::CommentsPublishedStatus;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let analytics_api = AnalyticsApi::new();
    ///     let response: Vec<CommentsPublishedStatus> = analytics_api
    ///         .get_comments_published().await;
    ///
    ///     println!("{:?}", response)
    /// }
    /// ```
    pub async fn get_comments_published(&self) -> Vec<CommentsPublishedStatus> {
        let response = self
            .tabnews_client
            .get("/analytics/child-content-published".to_owned())
            .await
            .unwrap();

        let json_response: Vec<CommentsPublishedStatus> = response.json().await.unwrap();

        json_response
    }
}
