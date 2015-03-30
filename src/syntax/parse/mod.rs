//! Parser

mod lexer;

use std::iter::Peekable;

use syntax::{Error, Error_};
use syntax::ast::{Expr, Expr_};
use syntax::codemap::{Source, Span, Spanned};
use syntax::parse::lexer::{Delim, Lexer, Token_};

struct Parser<'a> {
    lexer: Peekable<Lexer<'a>>,
    source: &'a Source,
    span: Span,
}

impl<'a> Parser<'a> {
    /// Parses the source code
    fn new(source: &'a Source) -> Parser<'a> {
        Parser {
            lexer: Lexer::new(source).peekable(),
            source: source,
            span: Span::dummy(),
        }
    }

    /// Parses an expression
    fn expr(&mut self) -> Result<Expr, Error> {
        match self.next() {
            None => unreachable!(),
            Some(Ok(Token_::Integer)) => self.integer(),
            Some(Ok(Token_::String)) => self.string(),
            Some(Ok(Token_::Symbol)) => self.symbol(),
            Some(Ok(Token_::Whitespace)) => self.expr(),
            Some(Err(error)) => Err(self.spanned(error)),
            Some(Ok(Token_::Open(Delim::Paren))) => self.list(),
            Some(Ok(Token_::Open(Delim::Bracket))) => self.vector(),
            _ => unimplemented!(),
        }
    }

    /// Parses an integer
    fn integer(&mut self) -> Result<Expr, Error> {
        let span = self.span;

        match self.source[span].parse() {
            Err(_) => Err(self.spanned(Error_::IntegerTooLarge)),
            Ok(integer) => Ok(self.spanned(Expr_::Integer(integer))),
        }
    }

    /// Parses a list
    fn list(&mut self) -> Result<Expr, Error> {
        Ok(try!(self.seq(Delim::Paren)).map(Expr_::List))
    }

    /// Advances the parser by one token
    fn next(&mut self) -> Option<Result<Token_, Error_>> {
        self.lexer.next().map(|result| {
            match result {
                Err(Spanned { span, node }) => {
                    self.span = span;
                    Err(node)
                }
                Ok(Spanned { span, node }) => {
                    self.span = span;
                    Ok(node)
                },
            }
        })
    }

    /// Parses a "sequence" until the `close` delimiter is reached
    fn seq(&mut self, close: Delim) -> Result<Spanned<Vec<Expr>>, Error> {
        let lo = self.span.lo;
        let mut exprs = vec![];

        loop {
            match self.lexer.peek() {
                None => {
                    return Err(Spanned::new(self.span.hi, self.span.hi, Error_::UnclosedDelimiter))
                },
                Some(&Err(error)) => {
                    self.next();
                    return Err(error)
                },
                Some(&Ok(Spanned { node: Token_::Close(delim), .. })) => {
                    self.next();

                    if delim == close {
                        break
                    } else {
                        return Err(self.spanned(Error_::IncorrectCloseDelimiter))
                    }
                },
                Some(&Ok(Spanned { node: Token_::Whitespace, .. })) => {
                    self.next();
                },
                Some(&Ok(_)) => {
                    exprs.push(try!(self.expr()));
                    continue
                },
            }
        }

        Ok(Spanned::new(lo, self.span.hi, exprs))
    }

    fn spanned<T>(&self, node: T) -> Spanned<T> {
        Spanned {
            node: node,
            span: self.span,
        }
    }

    /// Parses a string
    fn string(&self) -> Result<Expr, Error> {
        Ok(self.spanned(Expr_::String))
    }

    /// Parses a symbol
    fn symbol(&self) -> Result<Expr, Error> {
        Ok(self.spanned(Expr_::Symbol))
    }

    /// Parses a vector
    fn vector(&mut self) -> Result<Expr, Error> {
        Ok(try!(self.seq(Delim::Bracket)).map(Expr_::Vector))
    }
}

/// Parses a single expression
pub fn expr<'a>(source: &'a Source) -> Result<Expr, Error> {
    let mut parser = Parser::new(source);
    let expr = try!(parser.expr());

    loop {
        match parser.lexer.peek() {
            None => break,
            Some(&Ok(Spanned { node: Token_::Whitespace, .. })) => {
                parser.next();
            },
            Some(_) => {
                parser.next();
                return Err((parser.spanned(Error_::ExpectedEndOfLine)))
            }
        }
    }

    Ok(expr)
}
