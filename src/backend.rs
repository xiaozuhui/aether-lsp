use dashmap::DashMap;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer};

use crate::completion::get_completions;
use crate::diagnostics::DiagnosticEngine;
use crate::parser::{ParsedDocument, Parser};
use crate::symbols::SymbolTable;

pub struct AetherLspBackend {
    client: Client,
    documents: DashMap<String, ParsedDocument>,
}

/// Extract the word (identifier) at the given position
fn extract_word_at_position(text: &str, position: Position) -> Option<String> {
    let lines: Vec<&str> = text.lines().collect();
    if position.line as usize >= lines.len() {
        return None;
    }

    let line = lines[position.line as usize];
    let char_pos = position.character as usize;

    if char_pos > line.len() {
        return None;
    }

    // 找到单词的起始和结束位置
    let mut start = char_pos;
    let mut end = char_pos;

    // 向左查找单词开始
    while start > 0 {
        let ch = line.chars().nth(start - 1)?;
        if ch.is_alphanumeric() || ch == '_' {
            start -= 1;
        } else {
            break;
        }
    }

    // 向右查找单词结束
    while end < line.len() {
        let ch = line.chars().nth(end)?;
        if ch.is_alphanumeric() || ch == '_' {
            end += 1;
        } else {
            break;
        }
    }

    if start < end {
        Some(line[start..end].to_string())
    } else {
        None
    }
}

impl AetherLspBackend {
    pub fn new(client: Client) -> Self {
        AetherLspBackend {
            client,
            documents: DashMap::new(),
        }
    }

    async fn parse_and_diagnose(&self, uri: Url, text: String) {
        // 记录日志
        self.client
            .log_message(MessageType::INFO, format!("Parsing document: {}", uri))
            .await;

        let mut parser = Parser::new(&text);
        let parsed = parser.parse();

        // 生成诊断信息
        let diagnostics = DiagnosticEngine::analyze(&parsed, &text);

        // 记录诊断数量
        self.client
            .log_message(
                MessageType::INFO,
                format!("Found {} diagnostics for {}", diagnostics.len(), uri),
            )
            .await;

        // 缓存解析结果
        self.documents.insert(uri.to_string(), parsed);

        // 发送诊断信息到客户端
        self.client
            .publish_diagnostics(uri, diagnostics, None)
            .await;
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for AetherLspBackend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            server_info: Some(ServerInfo {
                name: "Aether LSP Server".to_string(),
                version: Some("0.1.0".to_string()),
            }),
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                completion_provider: Some(CompletionOptions {
                    resolve_provider: Some(false),
                    trigger_characters: Some(vec![".".to_string(), " ".to_string()]),
                    ..Default::default()
                }),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                definition_provider: Some(OneOf::Left(true)),
                document_symbol_provider: Some(OneOf::Left(true)),
                rename_provider: Some(OneOf::Left(true)),
                ..Default::default()
            },
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "Aether LSP Server initialized!")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        self.parse_and_diagnose(params.text_document.uri, params.text_document.text)
            .await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        if let Some(change) = params.content_changes.into_iter().next() {
            self.parse_and_diagnose(params.text_document.uri, change.text)
                .await;
        }
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        self.documents.remove(&params.text_document.uri.to_string());
    }

    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        let uri = params.text_document_position.text_document.uri.to_string();

        let completions = if let Some(doc) = self.documents.get(&uri) {
            get_completions(&doc, params.text_document_position.position)
        } else {
            get_completions(
                &ParsedDocument::default(),
                params.text_document_position.position,
            )
        };

        Ok(Some(CompletionResponse::Array(completions)))
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let uri = params
            .text_document_position_params
            .text_document
            .uri
            .to_string();
        let position = params.text_document_position_params.position;

        // 记录日志
        self.client
            .log_message(
                MessageType::INFO,
                format!(
                    "Hover requested at {}:{}",
                    position.line, position.character
                ),
            )
            .await;

        if let Some(doc) = self.documents.get(&uri) {
            // 先查找用户定义的符号
            if let Some(symbol_info) = doc.symbols.find_at_position(position) {
                return Ok(Some(Hover {
                    contents: HoverContents::Markup(MarkupContent {
                        kind: MarkupKind::Markdown,
                        value: symbol_info.documentation.clone(),
                    }),
                    range: Some(symbol_info.range),
                }));
            }

            // 查找内置函数 - 需要从文档中提取当前位置的标识符
            if let Some(word) = extract_word_at_position(&doc.text, position) {
                self.client
                    .log_message(MessageType::INFO, format!("Looking for builtin: {}", word))
                    .await;

                if let Some(builtin) = crate::builtins::find_builtin(&word) {
                    return Ok(Some(crate::builtins::builtin_to_hover(&builtin)));
                }
            }
        }

        Ok(None)
    }

    async fn goto_definition(
        &self,
        params: GotoDefinitionParams,
    ) -> Result<Option<GotoDefinitionResponse>> {
        let uri = params
            .text_document_position_params
            .text_document
            .uri
            .to_string();

        if let Some(doc) = self.documents.get(&uri) {
            let position = params.text_document_position_params.position;

            if let Some(location) = doc.symbols.find_definition(position) {
                return Ok(Some(GotoDefinitionResponse::Scalar(location)));
            }
        }

        Ok(None)
    }

    async fn document_symbol(
        &self,
        params: DocumentSymbolParams,
    ) -> Result<Option<DocumentSymbolResponse>> {
        let uri = params.text_document.uri.to_string();

        if let Some(doc) = self.documents.get(&uri) {
            let symbols = doc.symbols.to_document_symbols();
            return Ok(Some(DocumentSymbolResponse::Flat(symbols)));
        }

        Ok(None)
    }

    async fn rename(&self, params: RenameParams) -> Result<Option<WorkspaceEdit>> {
        let uri = params.text_document_position.text_document.uri.to_string();

        if let Some(doc) = self.documents.get(&uri) {
            let position = params.text_document_position.position;
            let new_name = params.new_name;

            // 验证新名称符合命名约定
            if !is_valid_aether_name(&new_name) {
                return Ok(None);
            }

            if let Some(edit) = doc.symbols.rename_symbol(position, &new_name, &uri) {
                return Ok(Some(edit));
            }
        }

        Ok(None)
    }
}

fn is_valid_aether_name(name: &str) -> bool {
    !name.is_empty()
        && name
            .chars()
            .all(|c| c.is_ascii_uppercase() || c == '_' || c.is_ascii_digit())
        && !name.chars().next().unwrap().is_ascii_digit()
}
