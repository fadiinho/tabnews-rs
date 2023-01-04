#[cfg(test)]
mod users_tests {
    use tabnews::internal::users::UsersApi;

    #[tokio::test]
    #[should_panic]
    async fn list_all_users() {
        let users_api = UsersApi::default();

        users_api.list_all_users().await.unwrap();
    }

    #[tokio::test]
    async fn get_user() {
        let users_api = UsersApi::default();

        let response = users_api.get_user("fadiinho").await.unwrap();

        assert!(!response.id.is_empty())
    }
}
