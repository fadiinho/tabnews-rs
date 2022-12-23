use crate::models;
use crate::models::content::ContentParams;
use crate::models::error::TabnewsError;
use crate::tabnews::client::TabnewsApi;

use models::content::Content;
use reqwest::Response;

pub struct PostsApi {
    tabnews_client: TabnewsApi,
}

impl PostsApi {
    pub fn new() -> Self {
        let tabnews_client = TabnewsApi::default();

        PostsApi { tabnews_client }
    }

    fn build_params(&self, params: Option<ContentParams>) -> Option<ContentParams> {
        let params = if params.is_some() {
            let p = params.unwrap();
            ContentParams { ..p }
        } else {
            ContentParams::default()
        };

        Some(params)
    }

    pub async fn get_homepage_posts(
        &self,
        params: Option<ContentParams>,
    ) -> Result<Vec<Content>, TabnewsError> {
        let _params = self.build_params(params);

        let response = self
            .tabnews_client
            .get("/contents".to_owned(), Some(&_params))
            .await
            .unwrap();

        let json_response: Vec<Content> = response.json().await.unwrap();

        Ok(json_response)
    }

    pub async fn get_posts_by_user(
        &self,
        username: &str,
        params: Option<ContentParams>,
    ) -> Result<Vec<Content>, TabnewsError> {
        let _params = self.build_params(params);

        let uri = format!("/contents/{}", username);

        let response = self.tabnews_client.get(uri, Some(&_params)).await.unwrap();

        let json_response: Vec<Content> = response.json().await.unwrap();

        Ok(json_response)
    }

    pub async fn get_post_details(
        &self,
        username: &str,
        slug: &str,
    ) -> Result<Content, TabnewsError> {
        let _params = ContentParams::default();
        let uri = format!("/contents/{}/{}", username, slug);

        let response = self.tabnews_client.get(uri, Some(&_params)).await.unwrap();

        let json_response = response.json().await.unwrap();

        Ok(json_response)
    }

    pub async fn get_post_comments(
        &self,
        username: &str,
        slug: &str,
    ) -> Result<Vec<Content>, TabnewsError> {
        let _params = ContentParams::default();
        let uri = format!("/contents/{}/{}/children", username, slug);

        let response = self.tabnews_client.get(uri, Some(&_params)).await.unwrap();

        let json_response = response.json().await.unwrap();

        Ok(json_response)
    }

    pub async fn get_post_thumbnail(
        &self,
        username: &str,
        slug: &str,
    ) -> Result<Response, TabnewsError> {
        let _params = ContentParams::default();
        let uri = format!("/contents/{}/{}/thumbnail", username, slug);

        let response = self.tabnews_client.get(uri, Some(&_params)).await.unwrap();

        Ok(response)
    }
}
