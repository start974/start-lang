use tower_lsp::{LspService, Server};

pub mod backend;
pub mod document;
pub mod interpreter;
mod position_memo;

pub async fn run() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();
    let (service, socket) = LspService::new(backend::Backend::from);
    Server::new(stdin, stdout, socket).serve(service).await;
}
