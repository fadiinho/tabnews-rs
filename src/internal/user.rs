use crate::models::user::User;

use std::cell::RefCell;
use std::rc::Rc;

use super::http_client::HttpClient;

pub struct UserApi {
    tabnews_client: Rc<RefCell<HttpClient>>,
}

impl Default for UserApi {
    fn default() -> Self {
        let tabnews_client = Rc::new(RefCell::new(HttpClient::default()));

        UserApi::new(tabnews_client)
    }
}

impl UserApi {
    pub fn new(client: Rc<RefCell<HttpClient>>) -> Self {
        UserApi {
            tabnews_client: client,
        }
    }

    /// Get current user information
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::collections::HashMap;
    ///
    /// use tabnews::TabnewsClient;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut headers = HashMap::new();
    ///
    ///     // session_id is required
    ///     headers.insert("cookie".to_owned(),
    ///                    "session_id=<session_id>".to_owned());
    ///
    ///     let client = TabnewsClient::new(headers);
    ///
    ///     let response = client.user_api.get_current_user().await.unwrap();
    ///
    ///     println!("{:?}", response);
    /// }
    /// ```
    // TODO: write tests
    pub async fn get_current_user(&self) -> Result<User, &str> {
        let _client = self.tabnews_client.borrow();
        if _client.get_header("Cookie").is_err() {
            return Err("`Cookie` header doesn't exists. In order to use `get_current_user()`, `cookie` with `session_id=<token>` value is required");
        }

        let response = _client.get("/user".to_owned()).await.unwrap();

        let json_response: User = response.json().await.unwrap();

        Ok(json_response)
    }
}
