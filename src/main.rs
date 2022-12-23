use tabnews_rs::tabnews::posts::PostsApi;

#[tokio::main]
pub async fn main() {
    let api = PostsApi::new();
    api.get_post_thumbnail("GabrielSozinho", "documentacao-da-api-do-tabnews")
        .await
        .unwrap();

    // println!("{:?}", response);
}
