//! Parser

mod lexer;

use std::iter::Peekable;

use syntax::ast::interner::Interner;
use syntax::ast::{Expr, Expr_};
use syntax::codemap::{Source, Span, Spanned};
use syntax::parse::lexer::{Delim, Lexer, Token_};
use syntax::{Error, Error_};

struct Parser<'a> {
    // NB `Option` needed for option dance
    interner: Option<&'a mut Interner>,
    lexer: Peekable<Lexer<'a>>,
    source: &'a Source,
    span: Span,
}

impl<'a> Parser<'a> {
    /// Parses the source code
    fn new(source: &'a Source, interner: &'a mut Interner) -> Parser<'a> {
        Parser {
            interner: Some(interner),
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
            Some(Ok(Token_::Keyword(_))) => Err(self.spanned(Error_::KeywordNotAllowedHere)),
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
        Ok(try!(self.seq(Delim::Paren, true)).map(Expr_::List))
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
    ///
    /// if `accept_keyword` is true, then the first element of the sequence can be a keyword
    fn seq(&mut self, close: Delim, accept_keyword: bool) -> Result<Spanned<Vec<Expr>>, Error> {
        let lo = self.span.lo;
        let mut exprs = vec![];

        loop {
            match self.lexer.peek() {
                None => {
                    let span = Span::new(self.span.hi, self.span.hi);

                    return Err(Spanned::new(span, Error_::UnclosedDelimiter))
                },
                Some(&Err(error)) => {
                    self.next();

                    return Err(error)
                },
                Some(&Ok(token)) => {
                    match token.node {
                        Token_::Close(delim) => {
                            self.next();

                            if delim == close {
                                break
                            } else {
                                return Err(self.spanned(Error_::IncorrectCloseDelimiter))
                            }
                        },
                        Token_::Keyword(keyword) if accept_keyword && exprs.len() == 0 => {
                            self.next();

                            exprs.push(self.spanned(Expr_::Keyword(keyword)));
                        },
                        Token_::Whitespace => {
                            self.next();
                        },
                        _ => {
                            exprs.push(try!(self.expr()))
                        }
                    }
                }
            }
        }

        let span = Span::new(lo, self.span.hi);

        Ok(Spanned::new(span, exprs))
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
    fn symbol(&mut self) -> Result<Expr, Error> {
        // NB option dance
        let interner = self.interner.take().unwrap();
        let string = &self.source[self.span];

        let expr = match string {
            "false" => Ok(self.spanned(Expr_::Bool(false))),
            "nil" => Ok(self.spanned(Expr_::Nil)),
            "true" => Ok(self.spanned(Expr_::Bool(true))),
            _ => Ok(self.spanned(Expr_::Symbol(interner.intern(string)))),
        };

        // NB option dance
        self.interner = Some(interner);

        expr
    }

    /// Parses a vector
    fn vector(&mut self) -> Result<Expr, Error> {
        Ok(try!(self.seq(Delim::Bracket, false)).map(Expr_::Vector))
    }
}

/// Parses a single expression
pub fn expr<'a>(source: &'a Source, interner: &'a mut Interner) -> Result<Expr, Error> {
    let mut parser = Parser::new(source, interner);
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
