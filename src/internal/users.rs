use crate::models::user::EditProfilePayload;
use std::collections::HashMap;

use crate::models::{error::TabnewsError, user::User};

use super::http_client::HttpClient;

pub struct UsersApi {
    tabnews_client: HttpClient,
}

impl Default for UsersApi {
    fn default() -> Self {
        let tabnews_client = HttpClient::
            default();

        UsersApi::new(tabnews_client)
    }
}

impl UsersApi {
    pub fn new(client: HttpClient) -> Self {
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

        let response = self
            .tabnews_client
            .post("/users".to_owned(), body)
            .await
            .unwrap();

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

        let response = self.tabnews_client.patch(url, payload).await.unwrap();

        // let json_response = response.json().await.unwrap();

        response
    }
}
