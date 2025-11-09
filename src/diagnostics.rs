//! Diagnostics engine for Aether code analysis

use crate::lexer::Lexer;
use crate::parser::{CompatParseError, ParsedDocument};
use crate::token::Token;
use tower_lsp::lsp_types::*;

pub struct DiagnosticEngine;

impl DiagnosticEngine {
    pub fn analyze(parsed: &ParsedDocument, text: &str) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();

        // 1. 检查语法错误（优先级最高）
        diagnostics.extend(Self::parse_errors_to_diagnostics(&parsed.errors));

        // 2. 检查命名约定（如果没有语法错误）
        if parsed.errors.is_empty() {
            diagnostics.extend(Self::check_naming_convention(text));
        }

        diagnostics
    }

    /// Convert parse errors to LSP diagnostics with precise location
    fn parse_errors_to_diagnostics(errors: &[CompatParseError]) -> Vec<Diagnostic> {
        errors
            .iter()
            .map(|error| {
                let start_pos = Position {
                    line: error.line.saturating_sub(1) as u32,
                    character: error.column.saturating_sub(1) as u32,
                };

                // Try to estimate error range based on message
                let error_length = Self::estimate_error_length(&error.message);

                Diagnostic {
                    range: Range {
                        start: start_pos,
                        end: Position {
                            line: start_pos.line,
                            character: start_pos.character + error_length,
                        },
                    },
                    severity: Some(DiagnosticSeverity::ERROR),
                    code: Some(NumberOrString::String(Self::error_code_from_message(
                        &error.message,
                    ))),
                    source: Some("aether-parser".to_string()),
                    message: error.message.clone(),
                    related_information: None,
                    tags: None,
                    code_description: None,
                    data: None,
                }
            })
            .collect()
    }

    /// Estimate error length for better range highlighting
    fn estimate_error_length(message: &str) -> u32 {
        if message.contains("identifier") {
            10
        } else if message.contains("Expected") {
            5
        } else if message.contains("UPPER_SNAKE_CASE") {
            15
        } else {
            8
        }
    }

    /// Extract error code from message
    fn error_code_from_message(message: &str) -> String {
        if message.contains("UPPER_SNAKE_CASE") {
            "E001".to_string()
        } else if message.contains("Unexpected token") {
            "E002".to_string()
        } else if message.contains("Expected") {
            "E003".to_string()
        } else if message.contains("Invalid expression") {
            "E004".to_string()
        } else {
            "E000".to_string()
        }
    }

    fn check_naming_convention(text: &str) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        let mut lexer = Lexer::new(text);
        let mut prev_token = Token::EOF;

        loop {
            let line = lexer.line();
            let column = lexer.column();
            let token = lexer.next_token();

            if token == Token::EOF {
                break;
            }

            // 检查标识符命名（跳过关键字后的标识符定义）
            if let Token::Identifier(name) = &token {
                // Skip checking for function/generator parameters (after Func/Generator keyword and parentheses)
                let is_definition = matches!(
                    prev_token,
                    Token::Set | Token::Func | Token::Generator | Token::Lazy
                );

                // Only check variable/function names, not all identifiers
                if is_definition && !Self::is_valid_aether_name(name) {
                    diagnostics.push(Diagnostic {
                        range: Range {
                            start: Position {
                                line: line.saturating_sub(1) as u32,
                                character: column.saturating_sub(1).max(0) as u32,
                            },
                            end: Position {
                                line: line.saturating_sub(1) as u32,
                                character: (column.saturating_sub(1) + name.len()) as u32,
                            },
                        },
                        severity: Some(DiagnosticSeverity::WARNING),
                        code: Some(NumberOrString::String("W001".to_string())),
                        code_description: Some(CodeDescription {
                            href: Url::parse(
                                "https://github.com/xiaozuhui/aether-lang/wiki/naming-conventions",
                            )
                            .unwrap_or_else(|_| Url::parse("file:///").unwrap()),
                        }),
                        source: Some("aether-lint".to_string()),
                        message: format!(
                            "变量名 '{}' 应使用 UPPER_SNAKE_CASE 格式\n建议: {}",
                            name,
                            Self::suggest_upper_snake_case(name)
                        ),
                        tags: None,
                        related_information: None,
                        data: None,
                    });
                }
            }

            prev_token = token;
        }

        diagnostics
    }

    /// Suggest UPPER_SNAKE_CASE version of a name
    fn suggest_upper_snake_case(name: &str) -> String {
        name.to_uppercase()
    }

    fn is_valid_aether_name(name: &str) -> bool {
        !name.is_empty()
            && name
                .chars()
                .all(|c| c.is_ascii_uppercase() || c == '_' || c.is_ascii_digit())
            && !name.chars().next().unwrap().is_ascii_digit()
    }
}
