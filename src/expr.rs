#![allow(unused_variables, dead_code)]
use crate::{Literal, Token};

#[derive(Debug)]
pub enum ExprKind {
    Assign {
        name: Token,
        value: Box<ExprKind>,
    },
    Binary {
        left: Box<ExprKind>,
        operator: Token,
        right: Box<ExprKind>,
    },
    Call {
        callee: Box<ExprKind>,
        token: Token,
        arguments: Vec<ExprKind>,
    },
    Get {
        object: Box<ExprKind>,
        name: Token,
    },
    Grouping {
        expression: Box<ExprKind>,
    },
    Literal {
        value: Literal,
    },
    Logical {
        left: Box<ExprKind>,
        operator: Token,
        right: Box<ExprKind>,
    },
    Set {
        object: Box<ExprKind>,
        name: Token,
        value: Box<ExprKind>,
    },
    Super {
        keyword: Token,
        method: Token,
    },
    This {
        keyword: Token,
    },
    Unary {
        operator: Token,
        right: Box<ExprKind>,
    },
    Variable {
        name: Token,
    },
}

#[derive(Debug)]
pub struct Expr {}
impl Expr {
    pub fn evaluate(data: &ExprKind) {
        match data {
            ExprKind::Assign { name, value } => {}
            ExprKind::Binary {
                left,
                operator,
                right,
            } => {}
            ExprKind::Call {
                callee,
                token,
                arguments,
            } => {}
            ExprKind::Get { object, name } => {}
            ExprKind::Grouping { expression } => {}
            ExprKind::Literal { value } => {}
            ExprKind::Logical {
                left,
                operator,
                right,
            } => {}
            ExprKind::Set {
                object,
                name,
                value,
            } => {}
            ExprKind::Super { keyword, method } => {}
            ExprKind::This { keyword } => {}
            ExprKind::Unary { operator, right } => {}
            ExprKind::Variable { name } => {}
        }
    }
}
