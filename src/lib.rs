//! A Lisp written in Rust

#![deny(missing_docs)]
#![deny(warnings)]
#![feature(into_cow)]

pub mod ast;
pub mod env;
pub mod eval;
pub mod parse;
