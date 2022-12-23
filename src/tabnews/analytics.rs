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

    pub async fn get_users_created(&self) -> Vec<UsersCreatedStatus> {
        let response = self
            .tabnews_client
            .get("/analytics/users-created".to_owned())
            .await
            .unwrap();

        let json_response: Vec<UsersCreatedStatus> = response.json().await.unwrap();

        json_response
    }

    pub async fn get_posts_published(&self) -> Vec<PostsPublishedStatus> {
        let response = self
            .tabnews_client
            .get("/analytics/root-content-published".to_owned())
            .await
            .unwrap();

        let json_response: Vec<PostsPublishedStatus> = response.json().await.unwrap();

        json_response
    }

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
