//! Pretty printing

use syntax::ast::{Expr, Expr_};
use syntax::codemap::Source;

/// Pretty prints an expression
pub fn expr(expr: &Expr, source: &Source) -> String {
    let mut string = String::new();
    expr_(&mut string, expr, source);
    string
}

fn expr_(string: &mut String, expr: &Expr, source: &Source) {
    fn seq(string: &mut String, exprs: &[Expr], source: &Source) {
        let mut is_first = true;

        for expr in exprs {
            if is_first {
                is_first = false;
            } else {
                string.push(' ');
            }

            expr_(string, expr, source)
        }
    }

    match expr.node {
        Expr_::Bool(bool) => string.push_str(&bool.to_string()),
        Expr_::Integer(integer) => string.push_str(&integer.to_string()),
        Expr_::List(ref exprs) => {
            string.push('(');
            seq(string, exprs, source);
            string.push(')');
        },
        Expr_::Nil => string.push_str("nil"),
        Expr_::Operator(_) => string.push_str(&source[expr.span]),
        Expr_::String => string.push_str(&source[expr.span]),
        Expr_::Symbol(_) => string.push_str(&source[expr.span]),
        Expr_::Vector(ref exprs) => {
            string.push('[');
            seq(string, exprs, source);
            string.push(']');
        },
    }
}
