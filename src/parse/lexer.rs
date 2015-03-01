//! Lexer

use std::fmt;
use std::iter::Peekable;
use std::str::CharIndices;

/// Lexer
pub struct Lexer<'a> {
    input: &'a str,
    iter: Peekable<CharIndices<'a>>,
    pos: usize,
    state: State,
}

impl<'a> Lexer<'a> {
    /// Lexes the input string
    pub fn new(input: &'a str) -> Lexer<'a> {
        let input = input.trim();

        Lexer {
            pos: 0,
            input: input,
            iter: input.char_indices().peekable(),
            state: State::Ok,
        }
    }

    /// Advances the lexer until the `pred` is false, returns the traveled `span`
    fn advance_while<P>(&mut self, mut pred: P) -> Span where
        P: FnMut(char) -> bool,
    {
        let start = self.pos;

        while let Some(&(_, c)) = self.peek() {
            if pred(c) {
                self.next_char();
            } else {
                break;
            }
        }

        let end = self.peek().map(|&(i, _)| i).unwrap_or(self.input.len());

        Span(start, end)
    }

    fn fatal<T>(&mut self, e: Error<'a>) -> Result<T, Error<'a>> {
        self.state = State::Error;

        Err(e)
    }

    fn next_char(&mut self) -> Option<char> {
        match self.state {
            State::Ok => {
                self.iter.next().map(|(i, c)| {
                    self.pos = i;
                    c
                })
            },
            _ => None,
        }
    }

    fn peek(&mut self) -> Option<&(usize, char)> {
        self.iter.peek()
    }

    fn slice(&self, span: Span) -> &'a str {
        &self.input[span.0..span.1]
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token<'a>, Error<'a>>;

    fn next(&mut self) -> Option<Result<Token<'a>, Error<'a>>> {
        self.next_char().map(|c| {
            match c {
                '"' => {
                    // TODO handle escaping
                    let start = self.pos;
                    self.next_char();
                    let span = self.advance_while(|c| c != '"');

                    if self.next_char().is_none() {
                        Err(Error::UnterminatedString(&self.input[start..]))
                    } else {
                        Ok(Token::Literal(Literal::String(self.slice(span))))
                    }
                },
                '(' => Ok(Token::OpenDelim(Delim::Paren)),
                ')' => Ok(Token::CloseDelim(Delim::Paren)),
                '*' => Ok(Token::Symbol(Symbol::Star)),
                '+' => Ok(Token::Symbol(Symbol::Plus)),
                '-' => Ok(Token::Symbol(Symbol::Minus)),
                '/' => Ok(Token::Symbol(Symbol::Slash)),
                '[' => Ok(Token::OpenDelim(Delim::Bracket)),
                ']' => Ok(Token::CloseDelim(Delim::Bracket)),
                c if c.is_digit(10) => {
                    let span = self.advance_while(|c| c.is_digit(10));
                    let string = self.slice(span);
                    match string.parse() {
                        Err(_) => self.fatal(Error::ParseInt(string)),
                        Ok(int) => Ok(Token::Literal(Literal::Integer(int))),
                    }
                },
                c if is_ident_start(c) => {
                    let span = self.advance_while(ident_continue);
                    Ok(Token::Ident(self.slice(span)))
                },
                c if is_whitespace(c) => {
                    self.advance_while(is_whitespace);
                    Ok(Token::Whitespace)
                },
                c => self.fatal(Error::UnknownStartOfToken(c))
            }
        }).or_else(|| {
            match self.state {
                State::Error => unreachable!(),
                State::Ok => {
                    self.state = State::Over;
                    Some(Ok(Token::Eof))
                },
                State::Over => None,
            }
        })
    }
}

/// Spans represent a region of code
#[derive(Copy, Debug)]
struct Span(usize, usize);

/// Delimiters
#[derive(Copy, Debug)]
pub enum Delim {
    /// `[` or `]`
    Bracket,
    /// `(` or `)`
    Paren,
}

/// Lexer errors
#[derive(Debug)]
pub enum Error<'a> {
    /// Unterminated string: `"abc`
    UnterminatedString(&'a str),
    /// Error parsing an integer
    ParseInt(&'a str),
    /// Unknown start of token
    UnknownStartOfToken(char),
}

/// Literals
#[derive(Copy, Debug)]
pub enum Literal<'a> {
    /// Integer
    Integer(u64),
    /// String
    String(&'a str),
}

enum State {
    Error,
    Ok,
    Over,
}

/// Symbols
#[derive(Copy, Debug)]
pub enum Symbol {
    /// `-`
    Minus,
    /// `+`
    Plus,
    /// `/`
    Slash,
    /// `*`
    Star,
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Symbol::Minus => f.write_str("-"),
            Symbol::Plus => f.write_str("+"),
            Symbol::Slash => f.write_str("/"),
            Symbol::Star => f.write_str("*"),
        }
    }
}

/// Tokens
#[derive(Copy, Debug)]
pub enum Token<'a> {
    /// A closing delimiter
    CloseDelim(Delim),
    /// End of file
    Eof,
    /// Identifier
    Ident(&'a str),
    /// Literal
    Literal(Literal<'a>),
    /// An opening delimiter
    OpenDelim(Delim),
    /// A symbol: `+`, `*`, etc
    Symbol(Symbol),
    /// Whitespace: spaces, tabs, newlines, etc
    Whitespace,
}

fn ident_continue(c: char) -> bool {
    match c {
        '0' ... '9' | 'A' ... 'Z' | '_' |  'a' ... 'z' | '-' => true,
        _ => false,
    }
}

fn is_ident_start(c: char) -> bool {
    match c {
        'A' ... 'Z' | '_' | 'a' ... 'z' => true,
        _ => false,
    }
}

fn is_whitespace(c: char) -> bool {
    match c {
        ' ' | '\t' | ',' => true,
        _ => false,
    }
}
