use crate::interpreter::Interpreter as _;

use super::interpreter::Interpreter;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer};

#[derive(Debug)]
pub struct Backend {
    client: Client,
    //document: HashMap<Url, Interpreter>,
}

impl Backend {
    /// make new backend with client
    pub fn new(client: Client) -> Self {
        Self { client }
    }
    /// get language server name
    pub fn name() -> &'static str {
        env!("CARGO_PKG_NAME")
    }

    /// version of language
    pub fn version() -> &'static str {
        env!("CARGO_PKG_VERSION")
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(
        &self,
        _: InitializeParams,
    ) -> tower_lsp::jsonrpc::Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                ..Default::default()
            },
            server_info: Some(ServerInfo {
                name: Self::name().to_string(),
                version: Some(Self::version().to_string()),
            }),
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "LSP démarré")
            .await;
    }

    async fn shutdown(&self) -> tower_lsp::jsonrpc::Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri;
        let mut interpreter = Interpreter::new(uri.clone(), params.text_document.text);
        interpreter.run();
        let diags = interpreter.diagnostics().to_vec();
        self.client.publish_diagnostics(uri, diags, None).await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri;

        if let Some(change) = params.content_changes.first() {
            let new_text = change.text.clone();

            let mut interpreter = Interpreter::new(uri.clone(), new_text);
            interpreter.run();
            let diags = interpreter.diagnostics().to_vec();

            self.client.publish_diagnostics(uri, diags, None).await;
        }
    }
}
