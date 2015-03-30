//! Evaluation

use std::fmt;

use eval::env::Env;
use syntax::ast::{Expr, Expr_};
use syntax::codemap::{Source, Spanned};

pub mod env;

/// Spanned error
pub type Error = Spanned<Error_>;

/// A function
pub type Function = Box<Fn(&[Value]) -> Option<Value>>;

/// Evaluation error
#[derive(Debug, PartialEq)]
pub enum Error_ {
    /// `()`
    EmptyList,
    /// `(1 2 3)`
    ExpectedSymbol,
    /// `(foo 1 2)`
    UndefinedSymbol,
    /// `(+ 1)`
    UnsupportedOperation,
}

/// A value
#[derive(Debug)]
pub enum Value {
    /// `123`
    Integer(i64),
    /// `"Hello, world!"`
    String(String),
    /// `+`
    Symbol(String),
    /// `[1 "two" [3]]`
    Vector(Vec<Value>),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Value::Integer(integer) => integer.fmt(f),
            Value::String(ref string) => string.fmt(f),
            Value::Symbol(ref symbol) => symbol.fmt(f),
            Value::Vector(ref elems) => {
                try!(f.write_str("["));

                let mut is_first = true;
                for elem in elems {
                    if is_first {
                        is_first = false;
                    } else {
                        try!(f.write_str(" "))
                    }

                    try!(elem.fmt(f))
                }

                f.write_str("]")
            }
        }
    }
}

/// Evaluates an expression
pub fn expr(expr: &Expr, source: &Source, env: &Env) -> Result<Value, Error> {
    match expr.node {
        Expr_::Integer(integer) => Ok(Value::Integer(integer)),
        Expr_::List(ref exprs) => match &exprs[..] {
            [] => Err(Spanned { span: expr.span, node: Error_::EmptyList }),
            [ref head, tail..] => match head.node {
                Expr_::Symbol => {
                    if let Some(function) = env.get(&source[head.span]) {
                        let mut args = Vec::with_capacity(tail.len());

                        for elem in tail {
                            args.push(try!(::eval::expr(elem, source, env)));
                        }

                        if let Some(value) = function(&args) {
                            Ok(value)
                        } else {
                            Err(Spanned { span: expr.span, node: Error_::UnsupportedOperation })
                        }
                    } else {
                        Err(Spanned { span: head.span, node: Error_::UndefinedSymbol })
                    }
                },
                _ => Err(Spanned { span: head.span, node: Error_::ExpectedSymbol }),
            },
        },
        Expr_::String => Ok(Value::String(String::from_str(&source[expr.span]))),
        Expr_::Symbol => {
            let symbol = &source[expr.span];

            if env.contains(symbol) {
                Ok(Value::Symbol(String::from_str(symbol)))
            } else {
                Err(Spanned { span: expr.span, node: Error_::UndefinedSymbol })
            }
        },
        Expr_::Vector(ref exprs) => {
            let mut elems = Vec::with_capacity(exprs.len());

            for expr in exprs {
                elems.push(try!(::eval::expr(expr, source, env)))
            }

            Ok(Value::Vector(elems))
        },
    }
}
