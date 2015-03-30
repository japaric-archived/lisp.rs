//! Lexer

use std::iter::Peekable;
use std::str::CharIndices;

use syntax::{Error, Error_};
use syntax::codemap::{BytePos, Source, Span, Spanned};

/// Lexer
pub struct Lexer<'a> {
    input: &'a str,
    iter: Peekable<CharIndices<'a>>,
    pos: BytePos,
}

impl<'a> Lexer<'a> {
    /// Lexes the source code
    pub fn new(source: &'a Source) -> Lexer<'a> {
        let input = source.as_str();

        Lexer {
            input: input,
            iter: input.char_indices().peekable(),
            pos: 0,
        }
    }

    /// Advances the lexer while the predicate holds true
    fn advance_while<P>(&mut self, mut predicate: P) where P: FnMut(char) -> bool {
        loop {
            match self.iter.peek() {
                None => break,
                Some(&(_, c)) => if !predicate(c) { break },
            }

            self.next();
        }
    }

    /// Raises an error
    fn error(&mut self, lo: BytePos, error: Error_) -> Result<Token, Error> {
        Err(self.spanned(lo, error))
    }

    /// Lexes an integer
    fn integer(&mut self) -> Result<Token, Error> {
        let lo = self.pos;

        self.advance_while(is_part_of_integer);

        Ok(self.spanned(lo, Token_::Integer))
    }

    /// Advances the lexer by one character
    fn next(&mut self) -> Option<char> {
        self.iter.next().map(|(i, c)| {
            self.pos = i;
            c
        })
    }

    /// Returns the byte position of the next character, or `input.len()` if there is nothing left
    /// to lex
    fn next_byte_pos(&mut self) -> BytePos {
        self.iter.peek().map(|&(i, _)| i).unwrap_or(self.input.len())
    }

    /// Lexes a token
    fn token(&mut self, token: Token_) -> Result<Token, Error> {
        let lo = self.pos;

        Ok(self.spanned(lo, token))
    }

    /// Returns a spanned node with a span that begins at `lo` and ends at `next_byte_pos()`
    fn spanned<T>(&mut self, lo: BytePos, node: T) -> Spanned<T> {
        Spanned {
            node: node,
            span: Span { lo: lo, hi: self.next_byte_pos() },
        }
    }

    /// Lexes a string
    fn string(&mut self) -> Result<Token, Error> {
        let lo = self.pos;

        match self.next() {
            None => self.error(lo, Error_::UnterminatedString),
            Some(_) => {
                self.advance_while(|c| c != '"');

                // eat `"`
                if self.next().is_none() {
                    self.error(lo, Error_::UnterminatedString)
                } else {
                    Ok(self.spanned(lo, Token_::String))
                }
            },
        }
    }

    /// Lexes a symbol
    fn symbol(&mut self) -> Result<Token, Error> {
        let lo = self.pos;

        self.advance_while(is_part_of_symbol);

        Ok(self.spanned(lo, Token_::Symbol))
    }

    /// Lexes whitespace
    fn whitespace(&mut self) -> Result<Token, Error> {
        let lo = self.pos;

        self.advance_while(is_whitespace);

        Ok(self.spanned(lo, Token_::Whitespace))
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token, Error>;

    fn next(&mut self) -> Option<Result<Token, Error>> {
        self.next().map(|c| {
            match c {
                '"' => self.string(),
                '(' => self.token(Token_::Open(Delim::Paren)),
                ')' => self.token(Token_::Close(Delim::Paren)),
                '[' => self.token(Token_::Open(Delim::Bracket)),
                ']' => self.token(Token_::Close(Delim::Bracket)),
                '{' => self.token(Token_::Open(Delim::Brace)),
                '}' => self.token(Token_::Close(Delim::Brace)),
                c if is_part_of_integer(c) => self.integer(),
                c if is_start_of_symbol(c) => self.symbol(),
                c if is_whitespace(c) => self.whitespace(),
                _ => {
                    let lo = self.pos;

                    self.error(lo, Error_::UnknownStartOfToken)
                },
            }
        })
    }
}

pub type Token = Spanned<Token_>;

/// Tokens
#[derive(Copy, Debug)]
pub enum Token_ {
    /// Closing delimiter: `]`
    Close(Delim),
    /// `123`
    Integer,
    /// Opening delimiter: `(`
    Open(Delim),
    /// `"Hello, world!"`
    String,
    /// `+`, `=`
    Symbol,
    /// ` ` or `\t`
    Whitespace,
}

/// Delimiters
#[derive(Copy, Debug, PartialEq)]
pub enum Delim {
    /// `{}`
    Brace,
    /// `[]`
    Bracket,
    /// `()`
    Paren,
}

/// Is this character a delimiter?
fn is_delim(c: char) -> bool {
    match c {
        '(' | ')' | '[' | ']' | '{' | '}' => true,
        _ => false,
    }
}

/// Is this character a part of an integer?
fn is_part_of_integer(c: char) -> bool {
    match c {
        '0'...'9' => true,
        _ => false
    }
}

/// Is this character a part of a symbol?
fn is_part_of_symbol(c: char) -> bool {
    match c {
        '"' | ';' | '\'' | '\\' => false,
        c if is_delim(c) => false,
        c if is_whitespace(c) => false,
        _ => true,
    }
}

/// Is this character the start of a symbol?
fn is_start_of_symbol(c: char) -> bool {
    !is_part_of_integer(c) && is_part_of_symbol(c)
}

/// Is this character whitespace?
fn is_whitespace(c: char) -> bool {
    match c {
        ' ' | '\t' | ',' => true,
        _ => false,
    }
}
