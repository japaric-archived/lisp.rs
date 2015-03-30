//! Abstract Syntax Tree

use syntax::codemap::Spanned;

/// A spanned expression
pub type Expr = Spanned<Expr_>;

/// An expression
#[derive(Debug)]
pub enum Expr_ {
    /// `123`
    Integer(i64),
    /// `(+ 1 2)`
    List(Vec<Expr>),
    /// `"Hello, world!"`
    String,
    /// `+`, `-`
    Symbol,
    /// `[1 "two" 3]`
    Vector(Vec<Expr>),
}
