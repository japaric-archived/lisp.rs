//! Evaluation of expressions

use std::fmt;

use ast::Expr;
use env::Env;

/// Evaluation errors
#[derive(Debug)]
pub enum Error {
    /// `()`
    EmptyList,
    /// `("abc" 1 2)`
    ExpectedSymbolGot(Expr),
    /// Symbol not yet defined
    UndefinedSymbol,
    /// `(+ 1)`
    UnsupportedOperation,
}

/// A value
pub enum Value {
    /// Integer: `123`
    Integer(u64),
    /// `()`
    Nil,
    /// String: `123`
    String(String),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Value::Integer(int) => int.fmt(f),
            Value::Nil => f.write_str("nil"),
            Value::String(ref s) => write!(f, "\"{}\"", s),
        }
    }
}

/// Evaluates an expression
pub fn expr(e: &Expr, env: &Env) -> Result<Value, Error> {
    match *e {
        Expr::Integer(int) => Ok(Value::Integer(int)),
        Expr::List(ref list) => match &**list {
            [] => Err(Error::EmptyList),
            [ref sym, exprs..] => match *sym {
                Expr::Symbol(ref sym) => match env.get(sym) {
                    None => Err(Error::UndefinedSymbol),
                    Some(f) => {
                        let mut args = Vec::with_capacity(exprs.len());

                        for expr in exprs {
                            args.push(try!(::eval::expr(expr, env)));
                        }

                        f(&args)
                    },
                },
                ref e => Err(Error::ExpectedSymbolGot(e.clone())),
            },
        },
        Expr::Nil => Ok(Value::Nil),
        Expr::String(ref s) => Ok(Value::String(s.clone())),
        // XXX What to do here? Is a symbol a value? Print it as e.g. "function `+`"?
        Expr::Symbol(_) => unimplemented!(),
    }
}
