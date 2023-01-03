use reqwest::Client;
use serde::Deserialize;

const GIT_REPO_API_URL: &'static str = "https://api.github.com/repos";
const TABNEWS_REPO: &'static str = "filipedeschamps/tabnews.com.br";
const QUERY: &'static str = "environment=preview&per_page=1";

#[derive(Deserialize, Debug)]
struct GithubDeploymentResponse {
    pub statuses_url: String,
}

#[derive(Deserialize, Debug)]
struct GithubStatusResponse {
    pub target_url: String,
}

// Thanks to https://github.com/Gustavosta/TabNews.py/blob/d91da137b31c45f1e8f958790aeb60cbb1acc877/tabnews/utils.py
pub async fn get_preview_url() -> String {
    let url = format!(
        "{}/{}/deployments?{}",
        GIT_REPO_API_URL, TABNEWS_REPO, QUERY
    );

    let response = Client::new()
        .get(url)
        .header("User-Agent", "Tabnews-RS")
        .send()
        .await
        .unwrap();

    let deployment_response: Vec<GithubDeploymentResponse> = response.json().await.unwrap();
    let url = deployment_response.first().unwrap();

    let response = Client::new()
        .get(url.statuses_url.to_owned())
        .header("User-Agent", "Tabnews-RS")
        .send()
        .await
        .unwrap();

    let json_response: Vec<GithubStatusResponse> = response.json().await.unwrap();

    json_response.first().unwrap().target_url.to_owned()
}
