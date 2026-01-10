use tower_lsp::lsp_types::*;

use super::TestContext;

fn test_error(diag: PublishDiagnosticsParams, uri: Url) {
    assert_eq!(diag.uri, uri);

    let mut ds = diag.diagnostics.into_iter();
    let d = ds.next().unwrap();
    assert_eq!(d.severity, Some(DiagnosticSeverity::ERROR));
    assert_eq!(d.range.start.line, 0);
    assert_eq!(d.range.start.character, 8);
    assert_eq!(d.range.end.line, 0);
    assert_eq!(d.range.end.character, 20);
    assert_eq!(d.message, "Type variable not_exist_ty not found.");

    let d = ds.next().unwrap();
    assert_eq!(d.severity, Some(DiagnosticSeverity::ERROR));
    assert_eq!(d.range.start.line, 2);
    assert_eq!(d.range.start.character, 9);
    assert_eq!(d.range.end.line, 2);
    assert_eq!(d.range.end.character, 22);
    assert_eq!(d.message, "Expression variable not_exist_var not found.");

    let d = ds.next().unwrap();
    assert_eq!(d.severity, Some(DiagnosticSeverity::ERROR));
    assert_eq!(d.range.start.line, 4);
    assert_eq!(d.range.start.character, 0);
    assert_eq!(d.range.end.line, 4);
    assert_eq!(d.range.end.character, 1);
    assert_eq!(d.message, "Parsing expect \"command\".");

    let d = ds.next().unwrap();
    assert_eq!(d.severity, Some(DiagnosticSeverity::ERROR));
    assert_eq!(d.range.start.line, 5);
    assert_eq!(d.range.start.character, 16);
    assert_eq!(d.range.end.line, 5);
    assert_eq!(d.range.end.character, 17);
    assert_eq!(d.message, "Parsing expect \":\" or \")\".");

    let d = ds.next().unwrap();
    assert_eq!(d.severity, Some(DiagnosticSeverity::ERROR));
    assert_eq!(d.range.start.line, 7);
    assert_eq!(d.range.start.character, 14);
    assert_eq!(d.range.end.line, 7);
    assert_eq!(d.range.end.character, 15);
    assert_eq!(d.message, "Lexer expected \"'\", found \".\".");

    assert!(ds.next().is_none(), "There should be no more diagnostics.");
}

#[tokio::test]
async fn did_open() {
    let mut ctx = TestContext::new("diagnostic");
    ctx.initialize().await;
    let text_document = ctx.document_item("main.st", include_str!("workspace/diagnostic/main.st"));
    ctx.notify::<notification::DidOpenTextDocument>(DidOpenTextDocumentParams { text_document })
        .await;
    let uri = ctx.doc_uri("main.st");

    let dp = ctx.recv::<PublishDiagnosticsParams>().await;
    test_error(dp, uri);
}

#[tokio::test]
async fn did_change() {
    let mut ctx = TestContext::new("diagnostic");
    ctx.initialize().await;
    let text_document = ctx.versioned_document("main.st");
    let change = TextDocumentContentChangeEvent {
        range: None,
        range_length: None,
        text: include_str!("workspace/diagnostic/main.st").to_string(),
    };

    ctx.notify::<notification::DidChangeTextDocument>(DidChangeTextDocumentParams {
        text_document,
        content_changes: vec![change.clone(), change],
    })
    .await;
    let uri = ctx.doc_uri("main.st");

    let dp = ctx.recv::<PublishDiagnosticsParams>().await;
    test_error(dp, uri.clone());
    let dp = ctx.recv::<PublishDiagnosticsParams>().await;
    test_error(dp, uri);
}

#[tokio::test]
async fn did_open_print() {
    let mut ctx = TestContext::new("diagnostic");
    ctx.initialize().await;
    let text_document = ctx.document_item("info.st", include_str!("workspace/diagnostic/info.st"));
    ctx.notify::<notification::DidOpenTextDocument>(DidOpenTextDocumentParams { text_document })
        .await;

    let uri = ctx.doc_uri("info.st");
    let diag = ctx.recv::<PublishDiagnosticsParams>().await;

    assert_eq!(diag.uri, uri);

    assert_eq!(diag.diagnostics.len(), 14);
    for d in diag.diagnostics {
        assert_eq!(d.severity, Some(DiagnosticSeverity::INFORMATION));
    }
}
