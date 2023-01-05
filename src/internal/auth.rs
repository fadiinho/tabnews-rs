use std::{cell::RefCell, collections::HashMap, rc::Rc, str::FromStr};

use reqwest::header::{HeaderName, HeaderValue};

use crate::models::user::UserSession;

use super::http_client::HttpClient;

pub struct AuthApi {
    tabnews_client: Rc<RefCell<HttpClient>>,
}

impl AuthApi {
    pub fn new(client: Rc<RefCell<HttpClient>>) -> Self {
        AuthApi {
            tabnews_client: client,
        }
    }

    pub async fn get_user_session(&self, email: &str, password: &str) -> UserSession {
        let mut data: HashMap<&str, &str> = HashMap::new();

        data.insert("email", email);
        data.insert("password", password);

        let _client = self.tabnews_client.borrow();

        let response = _client.post("/sessions".to_owned(), data).await.unwrap();

        let json_response = response.json().await.unwrap();

        json_response
    }

    pub async fn login(&self, email: &str, password: &str) -> UserSession {
        let session = self.get_user_session(email, password).await;

        {
            let mut _client = self.tabnews_client.borrow_mut();

            let cookie_value = format!("session_id={}", session.token);

            _client.add_header(
                HeaderName::from_str("Cookie").unwrap(),
                HeaderValue::try_from(cookie_value).unwrap(),
            );
        }

        session
    }
}
