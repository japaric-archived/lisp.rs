//! Parser

pub mod lexer;

use std::error::FromError;

use ast::Expr;
use self::lexer::{Delim, Lexer, Literal, Token};

/// Parser errors
#[derive(Debug)]
pub enum Error<'a> {
    /// Lexer error
    Lexer(lexer::Error<'a>),
    /// Unexpected token
    Unexpected(Token<'a>),
    /// Unmatched delimiter
    Unmatched(Delim),
}

impl<'a> FromError<lexer::Error<'a>> for Error<'a> {
    fn from_error(e: lexer::Error<'a>) -> Error<'a> {
        Error::Lexer(e)
    }
}

/// Parser
pub struct Parser<'a> {
    token: Token<'a>,
    iter: Lexer<'a>,
}

impl<'a> Parser<'a> {
    /// Parses a string
    fn new(input: &'a str) -> Result<Parser<'a>, Error<'a>> {
        let mut lexer = Lexer::new(input);

        match lexer.next() {
            None => unreachable!(),
            Some(tok) => {
                Ok(Parser {
                    iter: lexer,
                    token: try!(tok),
                })
            }
        }
    }

    /// Advances the parser by one token
    fn bump(&mut self) -> Result<(), Error<'a>> {
        match self.iter.next() {
            None => unreachable!(),
            Some(tok) => {
                self.token = try!(tok);
            }
        }

        Ok(())
    }

    /// Parses an expression
    fn parse_expr(&mut self) -> Result<Expr, Error<'a>> {
        match self.token {
            Token::CloseDelim(delim) => Err(Error::Unmatched(delim)),
            Token::Eof => Ok(Expr::Nil),
            Token::Literal(_) => Ok(self.parse_lit()),
            Token::OpenDelim(Delim::Paren) => self.parse_list(),
            Token::Symbol(_) => Ok(self.parse_sym()),
            _ => unimplemented!(),
        }
    }

    /// Parses current token as a literal
    fn parse_lit(&self) -> Expr {
        match self.token {
            Token::Literal(lit) => match lit {
                Literal::Integer(int) => Expr::Integer(int),
                Literal::String(s) => Expr::String(s.to_string()),
            },
            _ => unreachable!(),
        }
    }

    /// Parse a list, current token is `(`
    fn parse_list(&mut self) -> Result<Expr, Error<'a>> {
        let mut exprs = vec![];

        loop {
            try!(self.bump());

            match self.token {
                Token::CloseDelim(Delim::Paren) => return Ok(Expr::List(exprs)),
                Token::Eof => return Err(Error::Unmatched(Delim::Paren)),
                Token::Literal(_) => exprs.push(self.parse_lit()),
                Token::OpenDelim(Delim::Paren) => exprs.push(try!(self.parse_list())),
                Token::Symbol(_) => exprs.push(self.parse_sym()),
                Token::Whitespace => {}
                _ => unimplemented!(),
            }
        }
    }

    /// Parses current token as a symbol
    fn parse_sym(&self) -> Expr {
        match self.token {
            Token::Symbol(sym) => Expr::Symbol(sym.to_string()),
            _ => unreachable!(),
        }
    }
}

/// Keywords
pub enum Keyword {
    /// `false`
    False,
    /// `nil`
    Nil,
    /// `true`
    True,
}

impl Keyword {
    /// String representation of the keyword
    pub fn to_str(&self) -> &'static str {
        use self::Keyword::*;

        match *self {
            False => "false",
            Nil => "nil",
            True => "true",
        }
    }
}

/// Parses the string as an expression
pub fn expr<'a>(input: &'a str) -> Result<Expr, Error<'a>> {
    Parser::new(input).and_then(|mut p| p.parse_expr())
}
