//! Abstract Syntax Tree (AST) definitions for the Aether language

use serde::{Deserialize, Serialize};

pub type Program = Vec<Stmt>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Stmt {
    Set {
        name: String,
        value: Expr,
    },
    SetIndex {
        object: Box<Expr>,
        index: Box<Expr>,
        value: Expr,
    },
    FuncDef {
        name: String,
        params: Vec<String>,
        body: Vec<Stmt>,
    },
    GeneratorDef {
        name: String,
        params: Vec<String>,
        body: Vec<Stmt>,
    },
    LazyDef {
        name: String,
        expr: Expr,
    },
    Return(Expr),
    Yield(Expr),
    Break,
    Continue,
    While {
        condition: Expr,
        body: Vec<Stmt>,
    },
    For {
        var: String,
        iterable: Expr,
        body: Vec<Stmt>,
    },
    ForIndexed {
        index_var: String,
        value_var: String,
        iterable: Expr,
        body: Vec<Stmt>,
    },
    Switch {
        expr: Expr,
        cases: Vec<(Expr, Vec<Stmt>)>,
        default: Option<Vec<Stmt>>,
    },
    Import {
        names: Vec<String>,
        path: String,
        aliases: Vec<Option<String>>,
    },
    Export(String),
    Throw(Expr),
    Expression(Expr),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Expr {
    Number(f64),
    BigInteger(String),
    String(String),
    Boolean(bool),
    Null,
    Identifier(String),
    Array(Vec<Expr>),
    Dict(Vec<(String, Expr)>),
    Binary {
        left: Box<Expr>,
        op: BinOp,
        right: Box<Expr>,
    },
    Unary {
        op: UnaryOp,
        expr: Box<Expr>,
    },
    Call {
        func: Box<Expr>,
        args: Vec<Expr>,
    },
    Index {
        object: Box<Expr>,
        index: Box<Expr>,
    },
    If {
        condition: Box<Expr>,
        then_branch: Vec<Stmt>,
        elif_branches: Vec<(Expr, Vec<Stmt>)>,
        else_branch: Option<Vec<Stmt>>,
    },
    Lambda {
        params: Vec<String>,
        body: Vec<Stmt>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BinOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    And,
    Or,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UnaryOp {
    Minus,
    Not,
}

impl Expr {
    pub fn binary(left: Expr, op: BinOp, right: Expr) -> Self {
        Expr::Binary {
            left: Box::new(left),
            op,
            right: Box::new(right),
        }
    }

    pub fn unary(op: UnaryOp, expr: Expr) -> Self {
        Expr::Unary {
            op,
            expr: Box::new(expr),
        }
    }

    pub fn call(func: Expr, args: Vec<Expr>) -> Self {
        Expr::Call {
            func: Box::new(func),
            args,
        }
    }

    pub fn index(object: Expr, index: Expr) -> Self {
        Expr::Index {
            object: Box::new(object),
            index: Box::new(index),
        }
    }
}
