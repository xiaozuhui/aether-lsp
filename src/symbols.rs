//! Symbol table for tracking variables, functions, etc.

use crate::ast::{Expr, Program, Stmt};
use tower_lsp::lsp_types::*;

#[derive(Debug, Clone, Default)]
pub struct SymbolTable {
    pub variables: Vec<SymbolInfo>,
    pub functions: Vec<SymbolInfo>,
}

#[derive(Debug, Clone)]
pub struct SymbolInfo {
    pub name: String,
    pub kind: SymbolKind,
    pub range: Range,
    pub selection_range: Range,
    pub documentation: String,
    pub detail: Option<String>,
}

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable {
            variables: Vec::new(),
            functions: Vec::new(),
        }
    }

    /// Add a variable symbol to the table
    pub fn add_variable(&mut self, name: String, range: Range, detail: Option<String>) {
        self.variables.push(SymbolInfo {
            name,
            kind: SymbolKind::VARIABLE,
            range,
            selection_range: range,
            documentation: String::new(),
            detail,
        });
    }

    /// Add a function symbol to the table
    pub fn add_function(
        &mut self,
        name: String,
        range: Range,
        params: Vec<String>,
        detail: Option<String>,
    ) {
        let param_str = params.join(", ");
        self.functions.push(SymbolInfo {
            name: name.clone(),
            kind: SymbolKind::FUNCTION,
            range,
            selection_range: range,
            documentation: format!("Function: {}({})", name, param_str),
            detail,
        });
    }

    /// Extract symbols from AST
    pub fn from_ast(ast: &Program, text: &str) -> Self {
        let mut table = SymbolTable::new();

        for stmt in ast {
            extract_symbols_from_stmt(stmt, &mut table, text);
        }

        table
    }

    pub fn find_at_position(&self, position: Position) -> Option<&SymbolInfo> {
        // Check variables
        for var in &self.variables {
            if position_in_range(position, var.range) {
                return Some(var);
            }
        }

        // Check functions
        for func in &self.functions {
            if position_in_range(position, func.range) {
                return Some(func);
            }
        }

        None
    }

    pub fn find_definition(&self, position: Position) -> Option<Location> {
        if let Some(symbol) = self.find_at_position(position) {
            return Some(Location {
                uri: Url::parse("file:///dummy").unwrap(),
                range: symbol.range,
            });
        }
        None
    }

    pub fn to_document_symbols(&self) -> Vec<SymbolInformation> {
        let mut symbols = Vec::new();

        for var in &self.variables {
            symbols.push(SymbolInformation {
                name: var.name.clone(),
                kind: var.kind,
                tags: None,
                deprecated: None,
                location: Location {
                    uri: Url::parse("file:///dummy").unwrap(),
                    range: var.range,
                },
                container_name: None,
            });
        }

        for func in &self.functions {
            symbols.push(SymbolInformation {
                name: func.name.clone(),
                kind: func.kind,
                tags: None,
                deprecated: None,
                location: Location {
                    uri: Url::parse("file:///dummy").unwrap(),
                    range: func.range,
                },
                container_name: None,
            });
        }

        symbols
    }

    pub fn rename_symbol(
        &self,
        _position: Position,
        _new_name: &str,
        _uri: &str,
    ) -> Option<WorkspaceEdit> {
        // TODO: 实现重命名
        None
    }
}

/// Helper: Check if position is within range
fn position_in_range(pos: Position, range: Range) -> bool {
    if pos.line < range.start.line || pos.line > range.end.line {
        return false;
    }
    if pos.line == range.start.line && pos.character < range.start.character {
        return false;
    }
    if pos.line == range.end.line && pos.character > range.end.character {
        return false;
    }
    true
}

/// Extract symbols from a statement
fn extract_symbols_from_stmt(stmt: &Stmt, table: &mut SymbolTable, _text: &str) {
    match stmt {
        Stmt::Set { name, .. } => {
            // Estimate line 0 as placeholder - we'll improve this with line tracking
            let range = Range {
                start: Position {
                    line: 0,
                    character: 0,
                },
                end: Position {
                    line: 0,
                    character: name.len() as u32,
                },
            };
            table.add_variable(name.clone(), range, Some(format!("Variable: {}", name)));
        }
        Stmt::FuncDef { name, params, body } => {
            let range = Range {
                start: Position {
                    line: 0,
                    character: 0,
                },
                end: Position {
                    line: body.len() as u32,
                    character: 0,
                },
            };
            table.add_function(
                name.clone(),
                range,
                params.clone(),
                Some(format!(
                    "Function: {}({}) {{ ... }}",
                    name,
                    params.join(", ")
                )),
            );

            // Extract symbols from function body
            for body_stmt in body {
                extract_symbols_from_stmt(body_stmt, table, _text);
            }
        }
        Stmt::GeneratorDef { name, params, body } => {
            let range = Range {
                start: Position {
                    line: 0,
                    character: 0,
                },
                end: Position {
                    line: body.len() as u32,
                    character: 0,
                },
            };
            table.add_function(
                name.clone(),
                range,
                params.clone(),
                Some(format!(
                    "Generator: {}({}) {{ ... }}",
                    name,
                    params.join(", ")
                )),
            );

            for body_stmt in body {
                extract_symbols_from_stmt(body_stmt, table, _text);
            }
        }
        Stmt::LazyDef { name, .. } => {
            let range = Range {
                start: Position {
                    line: 0,
                    character: 0,
                },
                end: Position {
                    line: 0,
                    character: name.len() as u32,
                },
            };
            table.add_variable(name.clone(), range, Some(format!("Lazy: {}", name)));
        }
        Stmt::While { body, .. } | Stmt::For { body, .. } | Stmt::ForIndexed { body, .. } => {
            for body_stmt in body {
                extract_symbols_from_stmt(body_stmt, table, _text);
            }
        }
        Stmt::Switch { cases, default, .. } => {
            for (_, case_body) in cases {
                for case_stmt in case_body {
                    extract_symbols_from_stmt(case_stmt, table, _text);
                }
            }
            if let Some(default_body) = default {
                for default_stmt in default_body {
                    extract_symbols_from_stmt(default_stmt, table, _text);
                }
            }
        }
        Stmt::Expression(expr) => {
            extract_symbols_from_expr(expr, table, _text);
        }
        _ => {}
    }
}

/// Extract symbols from an expression (for nested lambdas, if expressions, etc.)
fn extract_symbols_from_expr(expr: &Expr, table: &mut SymbolTable, _text: &str) {
    match expr {
        Expr::Lambda { params, body } => {
            // Anonymous lambda - could track params if needed
            for body_stmt in body {
                extract_symbols_from_stmt(body_stmt, table, _text);
            }
        }
        Expr::If {
            then_branch,
            elif_branches,
            else_branch,
            ..
        } => {
            for stmt in then_branch {
                extract_symbols_from_stmt(stmt, table, _text);
            }
            for (_, elif_body) in elif_branches {
                for stmt in elif_body {
                    extract_symbols_from_stmt(stmt, table, _text);
                }
            }
            if let Some(else_body) = else_branch {
                for stmt in else_body {
                    extract_symbols_from_stmt(stmt, table, _text);
                }
            }
        }
        _ => {}
    }
}
