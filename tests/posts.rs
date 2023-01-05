#[cfg(test)]
mod posts_tests {
    use tabnews::internal::posts::PostsApi;
    use tabnews::models::content::ContentParams;

    #[tokio::test]
    async fn get_homepage_posts() {
        let posts_api = PostsApi::default();

        let response = posts_api.get_homepage_posts(None).await.unwrap();

        assert!(!response.is_empty())
    }

    #[tokio::test]
    async fn get_homepage_posts_with_params() {
        let params = ContentParams {
            per_page: Some(1),
            strategy: Some("old".to_string()),
            page: Some(1),
        };

        let posts_api = PostsApi::default();

        let response = posts_api.get_homepage_posts(Some(params)).await.unwrap();

        assert!(!response.is_empty())
    }

    #[tokio::test]
    async fn get_user_posts() {
        let posts_api = PostsApi::default();

        let response = posts_api.get_posts_by_user("fadiinho", None).await.unwrap();

        assert!(!response.is_empty())
    }

    #[tokio::test]
    #[should_panic]
    async fn get_non_existent_user_posts() {
        let posts_api = PostsApi::default();

        let response = posts_api
            .get_posts_by_user("non-existent-user", None)
            .await
            .unwrap();

        assert!(!response.is_empty())
    }

    #[tokio::test]
    async fn get_post_details() {
        let posts_api = PostsApi::default();
        let slug = "documentacao-da-api-do-tabnews";

        let response = posts_api
            .get_post_details("GabrielSozinho", slug)
            .await
            .unwrap();

        assert_eq!(response.slug, slug.to_owned())
    }

    #[tokio::test]
    #[should_panic]
    async fn get_non_existent_post_details() {
        let posts_api = PostsApi::default();
        let slug = "arroz";

        let response = posts_api.get_post_details("feijao", slug).await.unwrap();

        assert_eq!(response.slug, slug.to_owned())
    }

    #[tokio::test]
    async fn get_post_comments() {
        let posts_api = PostsApi::default();

        let response = posts_api
            .get_post_comments("GabrielSozinho", "documentacao-da-api-do-tabnews")
            .await
            .unwrap();

        assert!(!response.is_empty())
    }

    #[tokio::test]
    async fn get_post_thumbnail() -> Result<(), String> {
        let posts_api = PostsApi::default();

        let response = posts_api
            .get_post_thumbnail("GabrielSozinho", "documentacao-da-api-do-tabnews")
            .await
            .unwrap();

        match response.error_for_status() {
            Ok(_) => Ok(()),
            Err(_) => Err("Something went wrong!".to_owned()),
        }
    }

    #[tokio::test]
    async fn get_post_root() {
        let posts_api = PostsApi::default();

        let response = posts_api
            .get_post_root("GabrielSozinho", "ae53bb7c-264f-4ad1-aeb0-340dde9b1647")
            .await
            .unwrap();

        assert!(!response.id.is_empty())
    }

    #[tokio::test]
    async fn get_post_parent() {
        let posts_api = PostsApi::default();

        let response = posts_api
            .get_post_parent("GabrielSozinho", "ae53bb7c-264f-4ad1-aeb0-340dde9b1647")
            .await
            .unwrap();

        assert!(!response.id.is_empty())
    }

    #[tokio::test]
    async fn get_posts_rss() {
        let posts_api = PostsApi::default();

        let response = posts_api.get_rss().await.unwrap();

        assert!(!response.is_empty())
    }
}
