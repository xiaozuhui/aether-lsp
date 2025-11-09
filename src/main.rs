use tower_lsp::{LspService, Server};

mod ast;
mod backend;
mod builtins;
mod completion;
mod diagnostics;
mod lexer;
mod parser;
mod symbols;
mod token;

use backend::AetherLspBackend;

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| AetherLspBackend::new(client));

    Server::new(stdin, stdout, socket).serve(service).await;
}
