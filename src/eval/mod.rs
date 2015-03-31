//! Evaluation

use std::fmt;

use eval::env::Env;
use syntax::ast::{Expr, Expr_, Keyword};
use syntax::codemap::{Source, Spanned};

pub mod env;

/// Spanned error
pub type Error = Spanned<Error_>;

/// A built-in function
pub type Function = fn(&[Value]) -> Option<Value>;

impl Clone for Function {
    fn clone(&self) -> Function {
        *self
    }
}

impl fmt::Debug for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&(*self as *const ()), f)
    }
}

/// Evaluation error
#[derive(Debug, PartialEq)]
pub enum Error_ {
    /// `()`
    EmptyList,
    /// `(a 1 2)` where `a = 2`
    ExpectedFunction,
    /// `(1 2 3)`
    ExpectedSymbol,
    /// `(foo 1 2)`
    UndefinedSymbol,
    /// `(+ 1)`
    UnsupportedOperation,
}

/// A value
#[derive(Clone, Debug)]
pub enum Value {
    /// `true` or `false`
    Bool(bool),
    /// `+`
    Function(Function),
    /// `123`
    Integer(i64),
    ///  `nil`
    Nil,
    /// `"Hello, world!"`
    String(String),
    /// `[1 "two" [3]]`
    Vector(Vec<Value>),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Value::Bool(bool) => bool.fmt(f),
            Value::Function(function) => write!(f, "<function at {:?}>", function),
            Value::Integer(integer) => integer.fmt(f),
            Value::Nil => f.write_str("nil"),
            Value::String(ref string) => string.fmt(f),
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
pub fn expr(expr: &Expr, source: &Source, env: &mut Env) -> Result<Value, Error> {
    match expr.node {
        Expr_::Bool(bool) => Ok(Value::Bool(bool)),
        Expr_::Integer(integer) => Ok(Value::Integer(integer)),
        Expr_::Keyword(_) => {
            // This is a syntax error that gets caught earlier on
            unreachable!()
        },
        Expr_::List(ref exprs) => match &exprs[..] {
            [] => Err(Spanned::new(expr.span, Error_::EmptyList)),
            [ref head, tail..] => match head.node {
                Expr_::Keyword(keyword) => {
                    match keyword {
                        Keyword::Def => {
                            if let [ref symbol, ref expr] = tail {
                                if let Expr_::Symbol = symbol.node {
                                    let value = try!(::eval::expr(expr, source, env));
                                    let symbol = String::from_str(&source[symbol.span]);

                                    env.variables.insert(symbol, value.clone());

                                    Ok(value)
                                } else {
                                    Err(Spanned::new(symbol.span, Error_::ExpectedSymbol))
                                }
                            } else {
                                Err(Spanned::new(expr.span, Error_::UnsupportedOperation))
                            }
                        },
                        Keyword::If => {
                            if let [ref cond, ref then, ref els] = tail {
                                if match try!(::eval::expr(cond, source, env)) {
                                    Value::Bool(false) | Value::Nil => false,
                                    _ => true,
                                } {
                                    ::eval::expr(then, source, env)
                                } else {
                                    ::eval::expr(els, source, env)
                                }
                            } else {
                                Err(Spanned::new(expr.span, Error_::UnsupportedOperation))
                            }
                        },
                        _ => unimplemented!(),
                    }
                },
                Expr_::Symbol => {
                    let symbol = &source[head.span];

                    if let Some(function) = env.functions.get(symbol).map(Clone::clone) {
                        let mut args = Vec::with_capacity(tail.len());

                        for elem in tail {
                            args.push(try!(::eval::expr(elem, source, env)));
                        }

                        if let Some(value) = function(&args) {
                            Ok(value)
                        } else {
                            Err(Spanned::new(expr.span, Error_::UnsupportedOperation))
                        }
                    } else if env.variables.contains_key(symbol) {
                        Err(Spanned::new(head.span, Error_::ExpectedFunction))
                    } else {
                        Err(Spanned::new(head.span, Error_::UndefinedSymbol))
                    }
                },
                _ => Err(Spanned::new(head.span, Error_::ExpectedSymbol)),
            },
        },
        Expr_::Nil => Ok(Value::Nil),
        Expr_::String => Ok(Value::String(String::from_str(&source[expr.span]))),
        Expr_::Symbol => {
            let symbol = &source[expr.span];

            if let Some(value) = env.variables.get(symbol) {
                Ok(value.clone())
            } else if let Some(&function) = env.functions.get(symbol) {
                Ok(Value::Function(function))
            } else {
                Err(Spanned::new(expr.span, Error_::UndefinedSymbol))
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
