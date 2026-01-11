use crate::{
    error::ReefError,
    expr::{Expr, ExprKind, Value},
};

pub struct Interpreter {}
impl Interpreter {
    pub fn new() -> Self {
        Interpreter {}
    }
    pub fn interpret(&self, expr_kind: &ExprKind) -> Result<Value, ReefError> {
        let expr = Expr::new();
        let value = expr.evaluate(expr_kind);
        println!("{:?}", &value);
        value
    }
}
