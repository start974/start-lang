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
        let uri = params.text_document.uri.clone();
        let path = uri.to_file_path().unwrap();
        let interpreter = Interpreter::new(&path, params.text_document.text);
        let diags = interpreter.diagnostics();

        self.client
            .publish_diagnostics(uri, diags.to_vec(), None)
            .await;
    }
}
