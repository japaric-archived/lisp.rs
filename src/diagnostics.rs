//! Diagnostics

use eval;
use syntax::codemap::{Source, Spanned};
use syntax;

/// Diagnose evaluation error
pub fn eval(error: eval::Error, source: &Source) -> String {
    common(error, source, |error, string| {
        use eval::Error_::*;

        match error {
            EmptyList => string.push_str("empty list"),
            ExpectedFunction => string.push_str("expected function"),
            ExpectedSymbol => string.push_str("expected symbol"),
            UndefinedSymbol => string.push_str("undefined symbol"),
            UnsupportedOperation => string.push_str("unsupported operation"),
        }
    })
}

/// Diagnose syntax error
pub fn syntax(error: syntax::Error, source: &Source) -> String {
    common(error, source, |error, string| {
        use syntax::Error_::*;

        match error {
            ExpectedEndOfLine => string.push_str("expected end of line"),
            IncorrectCloseDelimiter => string.push_str("incorrect close delimiter"),
            IntegerTooLarge => string.push_str("integer literal is too large"),
            KeywordNotAllowedHere => string.push_str("keyword not allowed here"),
            UnclosedDelimiter => string.push_str("un-closed delimiter"),
            UnknownCharacterEscape => string.push_str("unknown character escape"),
            UnknownStartOfToken => string.push_str("unknown start of token"),
            UnterminatedString => string.push_str("unterminated string literal"),
        };
    })
}

/// Common diagnostic routine
fn common<E, F>(error: Spanned<E>, source: &Source, f: F) -> String where
    F: FnOnce(E, &mut String),
{
    // Check that this is not the dummy span
    debug_assert!(error.span.lo != 0 || error.span.hi != 0);

    let mut string = String::from_str("error: ");

    f(error.node, &mut string);

    string.push('\n');
    string.push_str(source.as_str());
    string.push('\n');

    let is_cjk = false;

    for _ in 0..source.as_str()[..error.span.lo].width(is_cjk) {
        string.push(' ');
    }

    string.push('^');

    if error.span.hi <= source.as_str().len() {
        for _ in 1..source[error.span].width(is_cjk) {
            string.push('~');
        }
    }

    string.push('\n');

    string
}
