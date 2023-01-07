use serde::{Deserialize, Serialize};

pub enum TabcoinsTransaction {
    Credit,
    Debit,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Tabcoins {
    pub tabcoins: i64,
}

/// Struct that represents a post/comment
///
/// # Examples
///
/// Example on how to create a post
/// ```rust
/// # use tabnews::models::content::Content;
/// # fn main() {
/// let post = Content::default()
///     .set_title("This is a Cool Title")
///     .set_body("Here comes text/markdown")
///     .set_source_url("https://example.com")
///     .set_slug("example-of-a-cool-post");
/// # }
/// ```

/// Example on how to create a comment
/// ```rust
/// # use tabnews::models::content::Content;
/// # fn main() {
/// let post = Content::default()
///     .set_title("This is a Cool Title")
///     .set_body("Here comes text/markdown")
///     .set_source_url("https://example.com")
///     .set_slug("example-of-a-cool-post")
///     .set_parent_id("0000-4002-8922-0000");
/// # }
/// ```
#[derive(Serialize, Deserialize, Debug)]
pub struct Content {
    pub id: Option<String>,
    pub parent_id: Option<String>,
    pub owner_id: Option<String>,
    pub slug: Option<String>,
    pub body: Option<String>,
    pub title: Option<String>,
    pub status: Option<String>,
    pub source_url: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub published_at: Option<String>,
    pub deleted_at: Option<String>,
    pub tabcoins: Option<i64>,
    pub owner_username: Option<String>,
    pub children_deep_count: Option<u64>,
    pub children: Option<Box<Vec<Content>>>,
}

impl Default for Content {
    fn default() -> Self {
        Self {
            id: None,
            parent_id: None,
            owner_id: None,
            slug: None,
            body: None,
            title: None,
            status: Some("published".to_owned()),
            source_url: None,
            created_at: None,
            updated_at: None,
            published_at: None,
            deleted_at: None,
            tabcoins: None,
            owner_username: None,
            children_deep_count: None,
            children: None,
        }
    }
}

impl Content {
    /// Sets the body
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use tabnews::models::content::Content;
    /// # fn main() {
    /// let post = Content::default()
    ///     .set_body("Here comes text/markdown");
    /// # }
    /// ```
    pub fn set_body(&mut self, body: &str) -> &mut Self {
        self.body = Some(body.to_owned());

        self
    }

    /// Sets the title
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use tabnews::models::content::Content;
    /// # fn main() {
    /// let post = Content::default()
    ///     .set_title("This is a Cool Title");
    /// # }
    /// ```
    pub fn set_title(&mut self, title: &str) -> &mut Self {
        self.title = Some(title.to_owned());

        self
    }

    /// Sets the source url
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use tabnews::models::content::Content;
    /// # fn main() {
    /// let post = Content::default()
    ///     .set_source_url("https://example.com");
    /// # }
    /// ```
    pub fn set_source_url(&mut self, source_url: &str) -> &mut Self {
        self.source_url = Some(source_url.to_owned());

        self
    }

    /// Sets the slug
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use tabnews::models::content::Content;
    /// # fn main() {
    /// let post = Content::default()
    ///     .set_slug("example-of-a-cool-post");
    /// # }
    /// ```
    pub fn set_slug(&mut self, slug: &str) -> &mut Self {
        self.slug = Some(slug.to_owned());

        self
    }

    /// Sets the parent id
    ///
    /// Required to post comments
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use tabnews::models::content::Content;
    /// # fn main() {
    /// let comment = Content::default()
    ///     .set_parent_id("0000-4002-8922-0000");
    /// # }
    /// ```
    pub fn set_parent_id(&mut self, parent_id: &str) -> &mut Self {
        self.parent_id = Some(parent_id.to_owned());

        self
    }
}

#[derive(Serialize)]
pub struct ContentParams {
    pub page: Option<u64>,
    pub per_page: Option<u64>,
    pub strategy: Option<String>,
}

impl Default for ContentParams {
    fn default() -> Self {
        ContentParams {
            page: None,
            per_page: None,
            strategy: None,
        }
    }
}
