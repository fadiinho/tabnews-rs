use reqwest::Response;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use super::http_client::HttpClient;

use crate::models::content::Content;
use crate::models::content::ContentParams;
use crate::models::content::Tabcoins;
use crate::models::content::TabcoinsTransaction;
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
    /// # use tabnews::TabnewsClient;
    /// # use tabnews::models::content::Content;
    /// # use tabnews::models::error::TabnewsError;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), TabnewsError> {
    /// let client = TabnewsClient::default();
    ///
    /// let homepage_posts: Vec<Content> = client.posts_api
    ///     .get_homepage_posts(None).await?;
    ///
    /// assert!(homepage_posts.len() >= 0);
    /// Ok(())
    /// # }
    /// ```
    /// Example with parameters:
    ///
    /// ```
    /// # use tabnews::TabnewsClient;
    /// # use tabnews::models::content::{Content, ContentParams};
    /// # use tabnews::models::error::TabnewsError;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), TabnewsError> {
    /// let client = TabnewsClient::default();
    ///
    /// let parameters = ContentParams {
    ///     page: Some(1),
    ///     per_page: Some(10),
    ///     strategy: Some("old".to_owned())
    /// };
    ///
    /// let homepage_posts: Vec<Content> = client.posts_api
    ///     .get_homepage_posts(Some(parameters)).await?;
    ///
    /// assert!(homepage_posts.len() >= 0);
    /// # Ok(())
    /// # }
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
    /// # use tabnews::TabnewsClient;
    /// # use tabnews::models::content::Content;
    /// # use tabnews::models::error::TabnewsError;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), TabnewsError> {
    /// let client = TabnewsClient::default();
    ///
    /// let fadiinho_posts: Vec<Content> = client.posts_api
    ///     .get_posts_by_user("fadiinho", None).await?;
    ///
    /// assert!(fadiinho_posts.len() >= 0);
    /// # Ok(())
    /// # }
    /// ```
    /// Example with parameters:
    ///
    /// ```
    /// # use tabnews::TabnewsClient;
    /// # use tabnews::models::content::{Content, ContentParams};
    /// # use tabnews::models::error::TabnewsError;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), TabnewsError> {
    /// let client = TabnewsClient::default();
    ///
    /// let parameters = ContentParams {
    ///     page: Some(1),
    ///     per_page: Some(10),
    ///     strategy: Some("old".to_owned())
    /// };
    ///
    /// let fadiinho_posts: Vec<Content> = client.posts_api
    ///     .get_posts_by_user("fadiinho", Some(parameters)).await?;
    ///
    /// assert!(fadiinho_posts.len() >= 0);
    /// # Ok(())
    /// # }
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
    /// # use tabnews::TabnewsClient;
    /// # use tabnews::models::content::Content;
    /// # use tabnews::models::error::TabnewsError;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), TabnewsError> {
    /// let client = TabnewsClient::default();
    ///
    /// let post: Content = client.posts_api.get_post_details(
    ///     "GabrielSozinho",
    ///     "documentacao-da-api-do-tabnews"
    /// ).await?;
    /// assert!(post.id.is_some());
    /// Ok(())
    /// # }
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
    /// # use tabnews::TabnewsClient;
    /// # use tabnews::models::content::{Content, ContentParams};
    /// # use tabnews::models::error::TabnewsError;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), TabnewsError> {
    /// let client = TabnewsClient::default();
    ///
    /// let post: Vec<Content> = client.posts_api.get_post_comments(
    ///     "GabrielSozinho",
    ///     "documentacao-da-api-do-tabnews"
    /// ).await?;
    /// assert!(post.len() >= 0);
    /// # Ok(())
    /// # }
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
    /// # use reqwest::Response;
    /// # use tabnews::TabnewsClient;
    /// # use tabnews::models::content::{Content, ContentParams};
    /// # use tabnews::models::error::TabnewsError;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), TabnewsError> {
    /// let client = TabnewsClient::default();
    ///
    /// let response: Response = client.posts_api.get_post_thumbnail(
    ///     "GabrielSozinho",
    ///     "documentacao-da-api-do-tabnews"
    /// ).await?;
    /// # Ok(())
    /// # }
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
    /// # use tabnews::TabnewsClient;
    /// # use tabnews::models::content::Content;
    /// # use tabnews::models::error::TabnewsError;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), TabnewsError> {
    /// let client = TabnewsClient::default();
    ///
    /// let post: Content = client.posts_api.get_post_parent(
    ///     "<username>",
    ///     "<children/comment slug>"
    /// ).await?;
    ///
    /// assert!(!post.id.is_some());
    /// # Ok(())
    /// # }
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
    /// # use tabnews::TabnewsClient;
    /// # use tabnews::models::content::Content;
    /// # use tabnews::models::error::TabnewsError;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), TabnewsError> {
    /// let client = TabnewsClient::default();
    ///
    /// let post: Content = client.posts_api.get_post_root(
    ///     "<username>",
    ///     "<children/comment slug>"
    /// ).await?;
    ///
    /// assert!(post.id.is_some());
    /// Ok(())
    /// # }
    /// ```
    pub async fn get_post_root(&self, username: &str, slug: &str) -> Result<Content, TabnewsError> {
        let uri = format!("/contents/{}/{}/root", username, slug);

        let _client = self.tabnews_client.borrow();

        let response = _client.get(uri).await.unwrap();

        let json_response = response.json().await.unwrap();

        Ok(json_response)
    }
    /// Get the tabcoins of a post
    ///
    /// # Examples
    ///
    /// ```
    /// # use tabnews::TabnewsClient;
    /// # use tabnews::models::error::TabnewsError;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), TabnewsError> {
    /// let client = TabnewsClient::default();
    ///
    /// let tabcoins: i64 = client.posts_api.get_post_tabcoins(
    ///     "GabrielSozinho",
    ///     "documentacao-da-api-do-tabnews"
    /// ).await?;
    ///
    /// assert!(
    ///     tabcoins < i64::MAX &&
    ///     tabcoins > i64::MIN
    /// );
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_post_tabcoins(&self, username: &str, slug: &str) -> Result<i64, TabnewsError> {
        let response = self.get_post_details(username, slug).await.unwrap();

        dbg!(&response);
        Ok(response.tabcoins.unwrap())
    }

    async fn _tabcoins_operation(
        &self,
        username: &str,
        slug: &str,
        transaction_type: TabcoinsTransaction,
    ) -> Result<Tabcoins, TabnewsError> {
        let uri = format!("/contents/{}/{}/tabcoins", username, slug);

        let _client = self.tabnews_client.borrow();

        let mut body: HashMap<&str, &str> = HashMap::new();

        body.insert(
            "transaction_type",
            match transaction_type {
                TabcoinsTransaction::Credit => "credit",
                TabcoinsTransaction::Debit => "debit",
            },
        );

        let response = _client.post(uri, body).await.unwrap();

        let json_response = response.json().await.unwrap();

        Ok(json_response)
    }

    // TODO: link to login docs
    /// Downvote a post.
    /// It will cost tabcoins of your account.
    ///
    /// # Panics
    /// It will panic if the cookie header isn't set.
    ///
    /// # Examples
    /// ```no_run
    /// # use tabnews::models::content::Tabcoins;
    /// # use tabnews::TabnewsClient;
    /// # use tabnews::models::error::TabnewsError;
    /// # #[tokio::main]
    /// # async fn main () -> Result<(), TabnewsError> {
    /// let client = TabnewsClient::default();
    ///
    /// let tabcoins: Tabcoins = client.posts_api.downvote(
    ///     "<username>",
    ///     "<post/comment slug>",
    /// ).await?;
    /// #     Ok(())
    /// # }
    /// ```
    pub async fn downvote(&self, username: &str, slug: &str) -> Result<Tabcoins, TabnewsError> {
        self._tabcoins_operation(username, slug, TabcoinsTransaction::Debit)
            .await
    }

    /// Upvote a post.
    /// It will cost tabcoins of your account.
    ///
    /// # Panics
    /// It will panic if the cookie header isn't set.
    ///
    /// # Examples
    /// ```no_run
    /// # use tabnews::models::content::Tabcoins;
    /// # use tabnews::TabnewsClient;
    /// # use tabnews::models::error::TabnewsError;
    /// # #[tokio::main]
    /// # async fn main () -> Result<(), TabnewsError> {
    /// let client = TabnewsClient::default();
    ///
    /// let tabcoins: Tabcoins = client.posts_api.upvote(
    ///     "<username>",
    ///     "<post/comment slug>",
    /// ).await?;
    /// #     Ok(())
    /// # }
    /// ```
    pub async fn upvote(&self, username: &str, slug: &str) -> Result<Tabcoins, TabnewsError> {
        self._tabcoins_operation(username, slug, TabcoinsTransaction::Credit)
            .await
    }

    /// Get TabNews RSS
    ///
    /// # Examples
    /// ```
    /// # use tabnews::TabnewsClient;
    /// # use tabnews::models::error::TabnewsError;
    /// # #[tokio::main]
    /// # async fn main () -> Result<(), TabnewsError> {
    /// let client = TabnewsClient::default();
    ///
    /// let rss: String = client.posts_api.get_rss().await?;
    ///
    /// assert!(!rss.is_empty());
    /// #     Ok(())
    /// # }
    /// ```
    pub async fn get_rss(&self) -> Result<String, TabnewsError> {
        let _client = self.tabnews_client.borrow();

        let response = _client.get("/contents/rss".to_owned()).await.unwrap();

        Ok(response.text().await.unwrap())
    }

    /// Publish a content
    async fn _publish(&self, content: Content) -> Result<Content, TabnewsError> {
        let _client = self.tabnews_client.borrow();

        let response = _client.post("/contents".to_owned(), content).await.unwrap();

        let json_response = response.json().await.unwrap();

        Ok(json_response)
    }

    /// Publish a post and returns it
    ///
    /// # Examples
    /// ```no_run
    /// # use tabnews::models::content::Content;
    /// # use tabnews::TabnewsClient;
    /// # #[tokio::main]
    /// # async fn main() {
    /// let client = TabnewsClient::default();
    /// let mut post = Content::default();
    ///
    /// post.set_title("Cool Title");
    /// post.set_body("Cool text/markdown");
    /// post.set_source_url("https://optional-source-url.com");
    /// post.set_slug("optional-slug");
    ///
    /// let response = client.posts_api.publish_post(post).await;
    ///
    /// assert!(response.id.is_some());
    /// # }
    /// ```
    pub async fn publish_post(&self, content: Content) -> Content {
        let response = self._publish(content).await.unwrap();

        response
    }

    /// Publish a comment and returns it
    ///
    /// # Panics
    ///
    /// It will panic if `parent_id` was not set
    ///
    /// # Examples
    /// ```no_run
    /// # use tabnews::models::content::Content;
    /// # use tabnews::TabnewsClient;
    /// # #[tokio::main]
    /// # async fn main()  {
    /// let client = TabnewsClient::default();
    /// let mut comment = Content::default();
    ///
    /// comment.set_title("Cool Title");
    /// comment.set_body("Cool text/markdown");
    /// comment.set_slug("optional-slug");
    /// comment.set_parent_id("<parent-id>");
    ///
    /// let response = client.posts_api.publish_comment(comment).await.unwrap();
    ///
    /// assert!(response.id.is_some());
    /// # }
    /// ```
    pub async fn publish_comment(&self, content: Content) -> Result<Content, &str> {
        if content.parent_id.is_none() {
            return Err("To post a comment `parent_id` must be set!");
        }

        let response = self._publish(content).await.unwrap();

        Ok(response)
    }
}
