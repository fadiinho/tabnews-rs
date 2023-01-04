use crate::models::user::EditProfilePayload;

use std::collections::HashMap;

use std::cell::RefCell;
use std::rc::Rc;

use crate::models::{error::TabnewsError, user::User};

use super::http_client::HttpClient;

pub struct UsersApi {
    tabnews_client: Rc<RefCell<HttpClient>>,
}

impl Default for UsersApi {
    fn default() -> Self {
        let tabnews_client = Rc::new(RefCell::new(HttpClient::default()));

        UsersApi::new(tabnews_client)
    }
}

impl UsersApi {
    pub fn new(client: Rc<RefCell<HttpClient>>) -> Self {
        UsersApi {
            tabnews_client: client,
        }
    }

    pub async fn create_user(
        &self,
        username: String,
        email: String,
        password: String,
    ) -> Result<User, TabnewsError> {
        let mut body: HashMap<String, String> = HashMap::new();
        body.insert("username".to_owned(), username);
        body.insert("email".to_owned(), email);
        body.insert("password".to_owned(), password);

        let _client = self.tabnews_client.borrow();

        let response = _client.post("/users".to_owned(), body).await.unwrap();

        let json_response: User = response.json().await.unwrap();

        Ok(json_response)
    }

    pub async fn edit_profile(
        &self,
        // TODO: We can only edit the profile of the current authenticated user
        // so maybe we can get the username from a "cached" info, then
        // the user won't need to set the `current_username`.
        current_username: String,
        payload: EditProfilePayload,
    ) -> reqwest::Response {
        let url = format!("users/{}", current_username);

        let _client = self.tabnews_client.borrow();

        let response = _client.patch(url, payload).await.unwrap();

        // let json_response = response.json().await.unwrap();

        response
    }

    /// List all users
    /// It can only be used by users that have permissions "read:user:list"
    /// ```no_run
    /// # use tabnews::TabnewsClient;
    /// # use tabnews::models::user::User;
    /// # use tabnews::models::error::TabnewsError;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), TabnewsError> {
    /// let client = TabnewsClient::default();
    /// let users: Vec<User>  = client.users_api.list_all_users().await?;
    ///     # Ok(())
    /// # }
    /// ```
    pub async fn list_all_users(&self) -> Result<Vec<User>, TabnewsError> {
        let _client = self.tabnews_client.borrow();

        let response = _client.get("/users".to_owned()).await.unwrap();

        let json_response = response.json().await.unwrap();

        Ok(json_response)
    }

    /// Get user info
    /// ```
    /// # use tabnews::TabnewsClient;
    /// # use tabnews::models::user::User;
    /// # use tabnews::models::error::TabnewsError;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), TabnewsError> {
    /// let client = TabnewsClient::default();
    /// let user: User  = client.users_api.get_user("fadiinho").await?;
    ///     # Ok(())
    /// # }
    /// ```
    pub async fn get_user(&self, username: &str) -> Result<User, TabnewsError> {
        let uri = format!("/users/{}", username);
        let _client = self.tabnews_client.borrow();

        let response = _client.get(uri).await.unwrap();

        let json_response = response.json().await.unwrap();

        Ok(json_response)
    }
}
