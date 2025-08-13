use super::TestContext;

#[tokio::test]
async fn initialize() {
    TestContext::new("simple").initialize().await;
}
