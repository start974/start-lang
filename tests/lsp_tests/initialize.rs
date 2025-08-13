use super::TestContext;

#[tokio::test]
async fn should_initialize() {
    TestContext::new("simple").initialize().await;
    // panic!("Don’t panic!");
}
