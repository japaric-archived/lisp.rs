//! Abstract Syntax Tree

use syntax::codemap::Spanned;

/// A spanned expression
pub type Expr = Spanned<Expr_>;

/// An expression
#[derive(Debug)]
pub enum Expr_ {
    /// `true` or `false`
    Bool(bool),
    /// `123`
    Integer(i64),
    /// `(+ 1 2)`
    List(Vec<Expr>),
    /// `def!`, `let*`
    Keyword(Keyword),
    /// `"Hello, world!"`
    String,
    /// `+`, `-`
    Symbol,
    /// `[1 "two" 3]`
    Vector(Vec<Expr>),
}

#[derive(Copy, Debug)]
/// Special atoms
pub enum Keyword {
    /// `def!`
    Def,
    /// `let*`
    Let,
}

impl Keyword {
    /// Checks if `str` is a keyword
    pub fn from_str(str: &str) -> Option<Keyword> {
        match str {
            "def!" => Some(Keyword::Def),
            "let*" => Some(Keyword::Let),
            _ => None,
        }
    }
}
