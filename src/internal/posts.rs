use reqwest::Response;

use std::cell::RefCell;
use std::rc::Rc;

use super::http_client::HttpClient;

use crate::models::content::Content;
use crate::models::content::ContentParams;
use crate::models::error::TabnewsError;

pub struct PostsApi {
    tabnews_client: Rc<RefCell<HttpClient>>,
}

impl Default for PostsApi {
    fn default() -> Self {
        let tabnews_client = Rc::new(RefCell::new(HttpClient::default()));

        PostsApi::new(tabnews_client)
    }
}

impl PostsApi {
    pub fn new(client: Rc<RefCell<HttpClient>>) -> Self {
        PostsApi {
            tabnews_client: client,
        }
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
    /// use tabnews::internal::posts::PostsApi;
    /// use tabnews::models::content::{Content, ContentParams};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let posts_api = PostsApi::default();
    ///
    ///     let homepage_posts: Vec<Content> = posts_api.get_homepage_posts(None).await.unwrap();
    ///
    ///     println!("{:?}", homepage_posts)
    /// }
    /// ```
    /// Example with parameters:
    ///
    /// ```
    /// use tabnews::internal::posts::PostsApi;
    /// use tabnews::models::content::{Content, ContentParams};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let posts_api = PostsApi::default();
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

        let _client = self.tabnews_client.borrow();

        let response = _client
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
    /// use tabnews::internal::posts::PostsApi;
    /// use tabnews::models::content::{Content, ContentParams};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let posts_api = PostsApi::default();
    ///
    ///     let fadiinho_posts: Vec<Content> = posts_api.get_posts_by_user("fadiinho", None).await.unwrap();
    ///
    ///     println!("{:?}", fadiinho_posts)
    /// }
    /// ```
    /// Example with parameters:
    ///
    /// ```
    /// use tabnews::internal::posts::PostsApi;
    /// use tabnews::models::content::{Content, ContentParams};
    ///
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let posts_api = PostsApi::default();
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

        let _client = self.tabnews_client.borrow();

        let response = _client.get_with_params(uri, Some(&_params)).await.unwrap();

        let json_response: Vec<Content> = response.json().await.unwrap();

        Ok(json_response)
    }

    /// Get the details of a specific post
    ///
    /// # Examples
    ///
    /// ```
    /// use tabnews::internal::posts::PostsApi;
    /// use tabnews::models::content::{Content, ContentParams};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let posts_api = PostsApi::default();
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

        let _client = self.tabnews_client.borrow();

        let response = _client.get_with_params(uri, Some(&_params)).await.unwrap();

        let json_response = response.json().await.unwrap();

        Ok(json_response)
    }

    /// Get the comments of a specific post
    ///
    /// # Examples
    ///
    /// ```
    /// use tabnews::internal::posts::PostsApi;
    /// use tabnews::models::content::{Content, ContentParams};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let posts_api = PostsApi::default();
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

        let _client = self.tabnews_client.borrow();

        let response = _client.get_with_params(uri, Some(&_params)).await.unwrap();

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
    /// use tabnews::internal::posts::PostsApi;
    /// use tabnews::models::content::{Content, ContentParams};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let posts_api = PostsApi::default();
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

        let _client = self.tabnews_client.borrow();

        let response = _client.get_with_params(uri, Some(&_params)).await.unwrap();

        Ok(response)
    }

    /// Get the parent of a specific post/comment
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use tabnews::internal::posts::PostsApi;
    /// use tabnews::models::content::Content;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let posts_api = PostsApi::default();
    ///     let post: Content = posts_api.get_post_parent(
    ///         "<username>",
    ///         "<children/comment slug>"
    ///     ).await.unwrap();
    ///
    ///     println!("{:?}", post)
    /// }
    /// ```
    pub async fn get_post_parent(
        &self,
        username: &str,
        slug: &str,
    ) -> Result<Content, TabnewsError> {
        let uri = format!("/contents/{}/{}/parent", username, slug);

        let _client = self.tabnews_client.borrow();

        let response = _client.get(uri).await.unwrap();

        let json_response = response.json().await.unwrap();

        Ok(json_response)
    }

    /// Get the root of a specific post/comment
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use tabnews::internal::posts::PostsApi;
    /// use tabnews::models::content::Content;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let posts_api = PostsApi::default();
    ///     let post: Content = posts_api.get_post_root(
    ///         "<username>",
    ///         "<children/comment slug>"
    ///     ).await.unwrap();
    ///
    ///     println!("{:?}", post)
    /// }
    /// ```
    pub async fn get_post_root(&self, username: &str, slug: &str) -> Result<Content, TabnewsError> {
        let uri = format!("/contents/{}/{}/root", username, slug);

        let _client = self.tabnews_client.borrow();

        let response = _client.get(uri).await.unwrap();

        let json_response = response.json().await.unwrap();

        Ok(json_response)
    }

    pub async fn get_post_tabcoins() {
        todo!("Not implemented!");
    }

    async fn _tab_coin_operation() {
        todo!("Not implemented!");
    }

    pub async fn downvote() {
        todo!("Not implemented!");
    }

    pub async fn upvote() {
        todo!("Not implemented!");
    }
}
