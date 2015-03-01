//! AST

use std::fmt;

use parse::lexer;

/// An expression
#[derive(Debug)]
pub enum Expr<'a> {
    /// Identifier: `foo`
    Ident(&'a str),
    /// Integer: `123`
    Integer(u64),
    /// List: `(+ 1 2)`
    List(Box<[Expr<'a>]>),
    /// `()`
    Nil,
    /// String: `"Hello, world!"`
    String(&'a str),
    /// Symbol: `+`
    Symbol(lexer::Symbol),
}

impl<'a> fmt::Display for Expr<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Expr::Ident(s) => f.write_str(s),
            Expr::Integer(int) => write!(f, "{}", int),
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
            Expr::String(s) => write!(f, "\"{}\"", s),
            Expr::Symbol(s) => s.fmt(f),
        }
    }
}
