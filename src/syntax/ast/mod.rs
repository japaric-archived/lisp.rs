//! Abstract Syntax Tree

pub mod interner;

use syntax::codemap::Spanned;

/// A spanned expression
pub type Expr = Spanned<Expr_>;

/// An interned symbol
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Symbol(usize);

/// An expression
#[derive(Debug)]
pub enum Expr_ {
    /// `true` or `false`
    Bool(bool),
    /// `123`
    Integer(i64),
    /// `(+ 1 2)`
    List(Vec<Expr>),
    /// `nil`
    Nil,
    /// `def!`, `let*`
    Operator(Operator),
    /// `"Hello, world!"`
    String,
    /// `+`, `-`
    Symbol(Symbol),
    /// `[1 "two" 3]`
    Vector(Vec<Expr>),
}

#[derive(Copy, Debug)]
/// Special operators
pub enum Operator {
    /// `def!`
    Def,
    /// `if`
    If,
    /// `let*`
    Let,
}

impl Operator {
    /// Checks if `str` is a special operator
    pub fn from_str(str: &str) -> Option<Operator> {
        match str {
            "def!" => Some(Operator::Def),
            "if" => Some(Operator::If),
            "let*" => Some(Operator::Let),
            _ => None,
        }
    }
}
