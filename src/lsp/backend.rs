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

impl Backend {
    async fn on_change(&self, params: TextDocumentItem) {
        self.client
            .log_message(MessageType::INFO, "LSP on change")
            .await;
        let uri = params.uri;
        let mut interpreter = Interpreter::new(uri.clone(), params.text);
        interpreter.run();
        let diags = interpreter.diagnostics().to_vec();
        self.client.publish_diagnostics(uri, diags, None).await;
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
            .log_message(MessageType::INFO, "LSP initialized")
            .await;
    }

    async fn shutdown(&self) -> tower_lsp::jsonrpc::Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        self.on_change(params.text_document).await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        self.on_change(TextDocumentItem {
            uri: params.text_document.uri,
            language_id: Self::name().to_string(),
            version: params.text_document.version,
            text: params
                .content_changes
                .first()
                .map_or(String::new(), |change| change.text.clone()),
        })
        .await;
    }
}
