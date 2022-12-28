use std::collections::HashMap;

use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue, IntoHeaderName, CONTENT_TYPE},
    Client, StatusCode,
};
use serde::Serialize;

use crate::models::error::TabnewsError;
pub const BASE_URL: &'static str = "https://www.tabnews.com.br/api/v1";

#[derive(Clone)]
pub struct HttpClient {
    host: &'static str,
    client: Client,
    headers: HeaderMap,
}

impl Default for HttpClient {
    fn default() -> Self {
        HttpClient::new(Some(BASE_URL))
    }
}

impl HttpClient {
    /// Creates a new [`TabnewsApi`].
    /// `host` can be passed as argument to change the default host
    pub fn new(host: Option<&'static str>) -> Self {
        let _host = match host {
            Some(h) => h,
            None => BASE_URL,
        };

        let client = Client::new();

        let mut headers = HeaderMap::new();

        headers.insert(
            CONTENT_TYPE,
            HeaderValue::from_str("application/json").unwrap(),
        );

        Self {
            host: _host,
            client,
            headers,
        }
    }

    pub async fn get_with_params<T: Serialize>(
        &self,
        path: String,
        params: Option<&T>,
    ) -> Result<reqwest::Response, TabnewsError> {
        let url = format!("{}/{}", self.host, path);

        let request = self
            .client
            .get(url.as_str())
            .query(params.unwrap())
            .headers(self.headers.to_owned());

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

    pub async fn get(&self, path: String) -> Result<reqwest::Response, TabnewsError> {
        let url = format!("{}/{}", self.host, path);

        let request = self
            .client
            .get(url.as_str())
            .headers(self.headers.to_owned());

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

    pub async fn post<T>(&self, path: String, body: T) -> Result<reqwest::Response, TabnewsError>
    where
        T: Serialize,
    {
        let url = format!("{}/{}", self.host, path);

        let request = self
            .client
            .post(url.as_str())
            .json(&body)
            .headers(self.headers.to_owned());

        let response = request.send().await.unwrap();

        Ok(response)
    }

    pub async fn patch<T>(&self, path: String, body: T) -> Result<reqwest::Response, TabnewsError>
    where
        T: Serialize,
    {
        let url = format!("{}/{}", self.host, path);

        println!("#http_client#patch#url {}", url);

        let request = self
            .client
            .patch(url.as_str())
            .json(&body)
            .headers(self.headers.to_owned());

        let response = request.send().await.unwrap();

        Ok(response)
    }

    pub fn add_header<K>(&mut self, key: K, value: HeaderValue)
    where
        K: IntoHeaderName,
    {
        self.headers.insert(key, value);
    }

    pub fn add_multiple_headers(&mut self, headers: HashMap<String, String>) {
        for (k, v) in headers.iter() {
            self.add_header(
                HeaderName::try_from(k).unwrap(),
                HeaderValue::try_from(v).unwrap(),
            );
        }
    }

    pub fn get_header(&self, header_key: &str) -> Result<&str, reqwest::header::ToStrError> {
        self.headers.get(header_key).unwrap().to_str()
    }
}
