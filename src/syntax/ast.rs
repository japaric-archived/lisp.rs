//! Abstract Syntax Tree

use syntax::codemap::Spanned;
use util::interner::Name;

/// A spanned expression
pub type Expr = Spanned<Expr_>;

/// An expression
#[derive(Debug)]
pub enum Expr_ {
    /// `true` or `false`
    Bool(bool),
    /// `123`
    Integer(i64),
    /// `:a`
    Keyword(Name),
    /// `(+ 1 2)`
    List(Vec<Expr>),
    /// `nil`
    Nil,
    /// `def!`, `let*`
    Operator(Operator),
    /// `"Hello, world!"`
    String,
    /// `+`, `-`
    Symbol(Name),
    /// `[1 "two" 3]`
    Vector(Vec<Expr>),
}

#[derive(Clone, Copy, Debug)]
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
