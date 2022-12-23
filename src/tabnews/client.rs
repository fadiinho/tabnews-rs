use reqwest::{Client, StatusCode};
use serde::Serialize;

use crate::models::error::TabnewsError;
pub const BASE_URL: &'static str = "https://www.tabnews.com.br/api/v1";

pub struct TabnewsApi {
    host: &'static str,
    client: Client,
}

impl Default for TabnewsApi {
    fn default() -> Self {
        TabnewsApi::new(Some(BASE_URL))
    }
}

impl TabnewsApi {
    /// Creates a new [`TabnewsApi`].
    /// `host` can be passed as argument to change the default host
    pub fn new(host: Option<&'static str>) -> Self {
        let _host = match host {
            Some(h) => h,
            None => BASE_URL,
        };

        let client = Client::new();

        Self {
            host: _host,
            client,
        }
    }

    pub async fn get_with_params<T: Serialize>(
        &self,
        path: String,
        params: Option<&T>,
    ) -> Result<reqwest::Response, TabnewsError> {
        let url = format!("{}/{}", self.host, path);

        let request = self.client.get(url.as_str()).query(params.unwrap());

        let response = request.send().await.unwrap();

        match response.status() {
            StatusCode::OK => {}
            _ => {
                let json_response: TabnewsError = response.json().await.unwrap();
                return Err(json_response);
            }
        }

        Ok(response)
    }
}
