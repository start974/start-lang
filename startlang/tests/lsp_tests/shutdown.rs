use super::TestContext;

#[tokio::test]
async fn shutdown() {
    let mut ctx = TestContext::new("simple");
    ctx.initialize().await;
    ctx.shutdown().await;
}
