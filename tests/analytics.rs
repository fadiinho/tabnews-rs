#[cfg(test)]
mod analytics_tests {
    use tabnews::internal::analytics::AnalyticsApi;

    #[tokio::test]
    async fn get_users_created_analytics() {
        let posts_api = AnalyticsApi::default();

        let response = posts_api.get_users_created().await;

        assert!(!response.is_empty())
    }

    #[tokio::test]
    async fn get_posts_published_analytics() {
        let posts_api = AnalyticsApi::default();

        let response = posts_api.get_posts_published().await;

        assert!(!response.is_empty())
    }

    #[tokio::test]
    async fn get_comments_published_analytics() {
        let posts_api = AnalyticsApi::default();

        let response = posts_api.get_comments_published().await;

        assert!(!response.is_empty())
    }
}
