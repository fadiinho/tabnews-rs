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

    /// Returns the posts of the homepage
    ///
    /// # Examples
    ///
    /// Example without parameters:
    ///
    /// ```
    /// use tabnews_rs::tabnews::posts::PostsApi;
    /// use tabnews_rs::models::content::{Content, ContentParams};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let posts_api = PostsApi::new();
    ///
    ///     let homepage_posts: Vec<Content> = posts_api.get_homepage_posts(None).await.unwrap();
    ///
    ///     println!("{:?}", homepage_posts)
    /// }
    /// ```
    /// Example with parameters:
    ///
    /// ```
    /// use tabnews_rs::tabnews::posts::PostsApi;
    /// use tabnews_rs::models::content::{Content, ContentParams};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let posts_api = PostsApi::new();
    ///
    ///     let parameters = ContentParams {
    ///         page: Some(1),
    ///         per_page: Some(10),
    ///         strategy: Some("old".to_owned())
    ///     };
    ///
    ///     let homepage_posts: Vec<Content> = posts_api.get_homepage_posts(Some(parameters)).await.unwrap();
    ///
    ///     println!("{:?}", homepage_posts)
    /// }
    /// ```
    pub async fn get_homepage_posts(
        &self,
        params: Option<ContentParams>,
    ) -> Result<Vec<Content>, TabnewsError> {
        let _params = self.build_params(params);

        let response = self
            .tabnews_client
            .get_with_params("/contents".to_owned(), Some(&_params))
            .await
            .unwrap();

        let json_response: Vec<Content> = response.json().await.unwrap();

        Ok(json_response)
    }

    /// Returns the posts of a specific user
    ///
    /// # Examples
    ///
    /// Example without parameters:
    /// ```
    /// use tabnews_rs::tabnews::posts::PostsApi;
    /// use tabnews_rs::models::content::{Content, ContentParams};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let posts_api = PostsApi::new();
    ///
    ///     let fadiinho_posts: Vec<Content> = posts_api.get_posts_by_user("fadiinho", None).await.unwrap();
    ///
    ///     println!("{:?}", fadiinho_posts)
    /// }
    /// ```
    /// Example with parameters:
    ///
    /// ```
    /// use tabnews_rs::tabnews::posts::PostsApi;
    /// use tabnews_rs::models::content::{Content, ContentParams};
    ///
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let posts_api = PostsApi::new();
    ///
    ///     let parameters = ContentParams {
    ///         page: Some(1),
    ///         per_page: Some(10),
    ///         strategy: Some("old".to_owned())
    ///     };
    ///
    ///     let fadiinho_posts: Vec<Content> = posts_api.get_posts_by_user("fadiinho", Some(parameters)).await.unwrap();
    ///
    ///     println!("{:?}", fadiinho_posts)
    /// }
    /// ```
    pub async fn get_posts_by_user(
        &self,
        username: &str,
        params: Option<ContentParams>,
    ) -> Result<Vec<Content>, TabnewsError> {
        let _params = self.build_params(params);

        let uri = format!("/contents/{}", username);

        let response = self
            .tabnews_client
            .get_with_params(uri, Some(&_params))
            .await
            .unwrap();

        let json_response: Vec<Content> = response.json().await.unwrap();

        Ok(json_response)
    }

    /// Get the details of a specific post
    ///
    /// # Examples
    ///
    /// ```
    /// use tabnews_rs::tabnews::posts::PostsApi;
    /// use tabnews_rs::models::content::{Content, ContentParams};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let posts_api = PostsApi::new();
    ///
    ///     let post: Content = posts_api.get_post_details(
    ///         "GabrielSozinho",
    ///         "documentacao-da-api-do-tabnews"
    ///     ).await.unwrap();
    ///
    ///     println!("{:?}", post)
    /// }
    /// ```
    pub async fn get_post_details(
        &self,
        username: &str,
        slug: &str,
    ) -> Result<Content, TabnewsError> {
        let _params = ContentParams::default();
        let uri = format!("/contents/{}/{}", username, slug);

        let response = self
            .tabnews_client
            .get_with_params(uri, Some(&_params))
            .await
            .unwrap();

        let json_response = response.json().await.unwrap();

        Ok(json_response)
    }

    /// Get the comments of a specific post
    ///
    /// # Examples
    ///
    /// ```
    /// use tabnews_rs::tabnews::posts::PostsApi;
    /// use tabnews_rs::models::content::{Content, ContentParams};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let posts_api = PostsApi::new();
    ///     let post: Vec<Content> = posts_api.get_post_comments(
    ///         "GabrielSozinho",
    ///         "documentacao-da-api-do-tabnews"
    ///     ).await.unwrap();
    ///
    ///     println!("{:?}", post)
    /// }
    /// ```
    pub async fn get_post_comments(
        &self,
        username: &str,
        slug: &str,
    ) -> Result<Vec<Content>, TabnewsError> {
        let _params = ContentParams::default();
        let uri = format!("/contents/{}/{}/children", username, slug);

        let response = self
            .tabnews_client
            .get_with_params(uri, Some(&_params))
            .await
            .unwrap();

        let json_response = response.json().await.unwrap();

        Ok(json_response)
    }

    /// Get the thumbnail of a specific post
    /// It returns a reqwest `Response`, that can be used to save the file.
    ///
    /// # Examples
    ///
    /// ```
    /// use reqwest::Response;
    /// use tabnews_rs::tabnews::posts::PostsApi;
    /// use tabnews_rs::models::content::{Content, ContentParams};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let posts_api = PostsApi::new();
    ///     let response: Response = posts_api.get_post_thumbnail(
    ///         "GabrielSozinho",
    ///         "documentacao-da-api-do-tabnews"
    ///     ).await.unwrap();
    ///
    ///     println!("{:?}", response)
    /// }
    /// ```
    pub async fn get_post_thumbnail(
        &self,
        username: &str,
        slug: &str,
    ) -> Result<Response, TabnewsError> {
        let _params = ContentParams::default();
        let uri = format!("/contents/{}/{}/thumbnail", username, slug);

        let response = self
            .tabnews_client
            .get_with_params(uri, Some(&_params))
            .await
            .unwrap();

        Ok(response)
    }
}
