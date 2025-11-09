//! Token definitions for the Aether language

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Token {
    // Keywords
    Set,
    Func,
    Return,
    If,
    Elif,
    Else,
    While,
    For,
    In,
    Break,
    Continue,
    Generator,
    Yield,
    Lazy,
    Force,
    Switch,
    Case,
    Default,
    Import,
    Export,
    From,
    As,
    Lambda,
    Throw,
    Try,
    Catch,

    // Literals
    Number(f64),
    BigInteger(String),
    String(String),
    Boolean(bool),
    Null,

    // Identifiers
    Identifier(String),

    // Operators
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    Assign,
    Equal,
    NotEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    And,
    Or,
    Not,
    Arrow,

    // Delimiters
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    Comma,
    Colon,
    Semicolon,
    Newline,

    // Special
    EOF,
    Illegal(char),
}

impl Token {
    /// Check if a string is a keyword, otherwise return it as an identifier
    pub fn lookup_keyword(ident: &str) -> Token {
        match ident {
            "Set" => Token::Set,
            "Func" => Token::Func,
            "Return" => Token::Return,
            "If" => Token::If,
            "Elif" => Token::Elif,
            "Else" => Token::Else,
            "While" => Token::While,
            "For" => Token::For,
            "In" => Token::In,
            "Break" => Token::Break,
            "Continue" => Token::Continue,
            "Generator" => Token::Generator,
            "Yield" => Token::Yield,
            "Lazy" => Token::Lazy,
            "Force" => Token::Force,
            "Switch" => Token::Switch,
            "Case" => Token::Case,
            "Default" => Token::Default,
            "Import" => Token::Import,
            "Export" => Token::Export,
            "From" => Token::From,
            "As" => Token::As,
            "Lambda" => Token::Lambda,
            "Throw" => Token::Throw,
            "Try" => Token::Try,
            "Catch" => Token::Catch,
            "True" => Token::Boolean(true),
            "False" => Token::Boolean(false),
            "Null" => Token::Null,
            _ => Token::Identifier(ident.to_string()),
        }
    }
}
