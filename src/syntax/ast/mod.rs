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
    /// `def!`, `let*`
    Keyword(Keyword),
    /// `(+ 1 2)`
    List(Vec<Expr>),
    /// `nil`
    Nil,
    /// `"Hello, world!"`
    String,
    /// `+`, `-`
    Symbol(Symbol),
    /// `[1 "two" 3]`
    Vector(Vec<Expr>),
}

#[derive(Copy, Debug)]
/// Special atoms
pub enum Keyword {
    /// `def!`
    Def,
    /// `if`
    If,
    /// `let*`
    Let,
}

impl Keyword {
    /// Checks if `str` is a keyword
    pub fn from_str(str: &str) -> Option<Keyword> {
        match str {
            "def!" => Some(Keyword::Def),
            "if" => Some(Keyword::If),
            "let*" => Some(Keyword::Let),
            _ => None,
        }
    }
}
