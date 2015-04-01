//! Syntax

pub mod ast;
pub mod codemap;
pub mod parse;
pub mod pp;

use syntax::codemap::Spanned;

/// A spanned error
pub type Error = Spanned<Error_>;

/// Syntax error
#[derive(Copy, Debug)]
pub enum Error_ {
    /// Only a single expression is expected per line. `(+ 1 2) 3` is an error
    ExpectedEndOfLine,
    /// `(+ 1 2]`
    IncorrectCloseDelimiter,
    /// The integer literal doesn't fit in 64 bits
    IntegerTooLarge,
    /// `(+ def! 1)`
    OperatorNotAllowedHere,
    /// `(+ 1 2`
    UnclosedDelimiter,
    /// `"\a"`
    UnknownCharacterEscape,
    /// No known token starts with this character
    UnknownStartOfToken,
    /// `"Hello`
    UnterminatedString,
}
