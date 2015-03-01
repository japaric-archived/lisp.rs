//! A Lisp written in Rust

#![deny(missing_docs)]
#![deny(warnings)]
#![feature(unicode)]

pub mod ast;
pub mod env;
pub mod eval;
pub mod parse;
