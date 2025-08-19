use crate::lsp_tests::test_context::TestContext;
use tower_lsp::lsp_types::{notification, request, *};

#[tokio::test]
async fn hover() {
    let mut ctx = TestContext::new("hover");
    ctx.initialize().await;
    let text_document = ctx.document_item("main.st", include_str!("workspace/hover/main.st"));
    ctx.notify::<notification::DidOpenTextDocument>(DidOpenTextDocumentParams { text_document })
        .await;
    let uri = ctx.doc_uri("main.st");

    let dp = ctx.recv::<PublishDiagnosticsParams>().await;
    assert_eq!(dp.diagnostics.len(), 0);

    let sep = MarkedString::from_markdown("-----".to_string());
    let hover_ty = ctx
        .request::<request::HoverRequest>(HoverParams {
            text_document_position_params: TextDocumentPositionParams {
                text_document: TextDocumentIdentifier { uri: uri.clone() },
                position: Position {
                    line: 1,
                    character: 5,
                },
            },
            work_done_progress_params: WorkDoneProgressParams::default(),
        })
        .await
        .unwrap();

    assert_eq!(
        hover_ty.contents,
        HoverContents::Array(vec![
            MarkedString::from_language_code("startlang".to_string(), "N0 := Nat".to_string()),
            sep.clone(),
            MarkedString::from_markdown("Documentation of `N0`.\n".to_string()),
        ])
    );

    assert_eq!(
        hover_ty.range,
        Some(Range {
            start: Position {
                line: 1,
                character: 5
            },
            end: Position {
                line: 1,
                character: 7
            }
        })
    );

    let hover_def_doc = ctx
        .request::<request::HoverRequest>(HoverParams {
            text_document_position_params: TextDocumentPositionParams {
                text_document: TextDocumentIdentifier { uri: uri.clone() },
                position: Position {
                    line: 4,
                    character: 5,
                },
            },
            work_done_progress_params: WorkDoneProgressParams::default(),
        })
        .await
        .unwrap();

    assert_eq!(
        hover_def_doc.contents,
        HoverContents::Array(vec![
            MarkedString::from_language_code("startlang".to_string(), "a : N0".to_string()),
            sep.clone(),
            MarkedString::from_markdown("Documentation of `a`.\n".to_string()),
        ])
    );

    assert_eq!(
        hover_def_doc.range,
        Some(Range {
            start: Position {
                line: 4,
                character: 4
            },
            end: Position {
                line: 4,
                character: 5
            }
        })
    );

    let hover_def = ctx
        .request::<request::HoverRequest>(HoverParams {
            text_document_position_params: TextDocumentPositionParams {
                text_document: TextDocumentIdentifier { uri: uri.clone() },
                position: Position {
                    line: 6,
                    character: 5,
                },
            },
            work_done_progress_params: WorkDoneProgressParams::default(),
        })
        .await
        .unwrap();

    assert_eq!(
        hover_def.contents,
        HoverContents::Scalar(MarkedString::from_language_code(
            "startlang".to_string(),
            "b : N0".to_string()
        ))
    );

    assert_eq!(
        hover_def.range,
        Some(Range {
            start: Position {
                line: 6,
                character: 4
            },
            end: Position {
                line: 6,
                character: 5
            }
        })
    );

    assert_eq!(
        hover_def_doc.range,
        Some(Range {
            start: Position {
                line: 4,
                character: 4
            },
            end: Position {
                line: 4,
                character: 5
            }
        })
    );

    let hover_def_doc2 = ctx
        .request::<request::HoverRequest>(HoverParams {
            text_document_position_params: TextDocumentPositionParams {
                text_document: TextDocumentIdentifier { uri: uri.clone() },
                position: Position {
                    line: 6,
                    character: 9,
                },
            },
            work_done_progress_params: WorkDoneProgressParams::default(),
        })
        .await
        .unwrap();

    assert_eq!(
        hover_def_doc2.contents,
        HoverContents::Array(vec![
            MarkedString::from_language_code("startlang".to_string(), "a : N0".to_string()),
            sep.clone(),
            MarkedString::from_markdown("Documentation of `a`.\n".to_string()),
        ])
    );

    assert_eq!(
        hover_def_doc2.range,
        Some(Range {
            start: Position {
                line: 6,
                character: 9
            },
            end: Position {
                line: 6,
                character: 10
            }
        })
    );

    let hover_nothing = ctx
        .request::<request::HoverRequest>(HoverParams {
            text_document_position_params: TextDocumentPositionParams {
                text_document: TextDocumentIdentifier { uri: uri.clone() },
                position: Position {
                    line: 6,
                    character: 2,
                },
            },
            work_done_progress_params: WorkDoneProgressParams::default(),
        })
        .await;

    assert!(hover_nothing.is_none());
}
