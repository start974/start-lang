use tower_lsp::lsp_types::*;

use super::TestContext;

#[tokio::test]
async fn did_open() {
    let mut ctx = TestContext::new("diagnostic");
    ctx.initialize().await;
    let text_document = ctx.document_item("main.st", include_str!("workspace/diagnostic/main.st"));
    ctx.notify::<notification::DidOpenTextDocument>(DidOpenTextDocumentParams { text_document }).await;

    let dp = ctx.recv::<PublishDiagnosticsParams>().await;
    assert_eq!(dp.uri, ctx.doc_uri("main.st"));
    let ds = dp.diagnostics;
    assert_eq!(ds.len(), 6);

    let d = &ds[0];
    assert_eq!(d.range.start.line, 0);
    assert_eq!(d.range.start.character, 8);
    assert_eq!(d.range.end.line, 0);
    assert_eq!(d.range.end.character, 20);
    assert_eq!(d.message, "Variable Not_exist_ty not found in the current scope.");


    //let d = &ds[1];
    //assert_eq!(d.range.start.line, 3);
    //assert_eq!(d.range.start.character, 9);
    //assert_eq!(d.range.end.line, 3);
    //assert_eq!(d.range.end.character, 24);
    //assert_eq!(d.message, "Variable Not_exist_var not found in current scope");
}
