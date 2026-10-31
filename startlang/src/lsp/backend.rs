use super::interpreter::Interpreter;
use crate::interpreter::Interpreter as _;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer};

#[derive(Debug)]
pub struct Backend {
    client: Client,
}

impl From<Client> for Backend {
    fn from(client: Client) -> Self {
        Self { client }
    }
}
impl Backend {
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
    /// on_change is called when the document is opened or changed
    async fn on_change(&self, params: TextDocumentItem) {
        self.client
            .log_message(MessageType::INFO, "LSP on change")
            .await;

        let uri = params.uri.clone();
        let diags = tokio::task::spawn_blocking(move || {
            let mut interpreter = Interpreter::new(&uri, params.text);
            interpreter.run();
            interpreter.diagnostics().to_vec()
        })
        .await
        .expect("the interpreter task panicked");

        self.client
            .publish_diagnostics(params.uri, diags, Some(params.version))
            .await;
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
        for change in params.content_changes {
            self.client
                .log_message(MessageType::INFO, &format!("LSP change: {}", change.text))
                .await;
            self.on_change(TextDocumentItem {
                uri: params.text_document.uri.clone(),
                language_id: Self::name().to_string(),
                version: params.text_document.version,
                text: change.text,
            })
            .await;
        }
    }
}
