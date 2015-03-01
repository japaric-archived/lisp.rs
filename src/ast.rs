//! AST

use std::fmt;

/// An expression
#[derive(Clone, Debug)]
pub enum Expr {
    /// Integer: `123`
    Integer(u64),
    /// List: `(+ 1 2)`
    List(Vec<Expr>),
    /// `()`
    Nil,
    /// String: `"Hello, world!"`
    String(String),
    /// Symbol: `+`
    Symbol(String),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Expr::Integer(int) => int.fmt(f),
            Expr::List(ref exprs) => {
                let mut is_first = true;

                for expr in exprs.iter() {
                    if is_first {
                        try!(f.write_str("("));
                        try!(expr.fmt(f));
                        is_first = false;
                    } else {
                        try!(f.write_str(" "));
                        try!(expr.fmt(f));
                    }
                }

                f.write_str(")")
            },
            Expr::Nil => f.write_str("nil"),
            Expr::String(ref s) => write!(f, "\"{}\"", s),
            Expr::Symbol(ref s) => s.fmt(f),
        }
    }
}
