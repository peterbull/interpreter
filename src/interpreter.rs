use crate::expr::{Expr, ExprKind};

pub struct Interpreter {}
impl Interpreter {
    pub fn eval_literal(expr: &ExprKind) {
        Expr::evaluate(&expr);
    }
}
