//! A Lisp interpreter written in Rust

#![deny(missing_docs)]
#![deny(warnings)]
#![feature(collections)]
#![feature(core)]
#![feature(slice_patterns)]
#![feature(unboxed_closures)]

extern crate rc;
extern crate unicode_width;

pub mod diagnostics;
pub mod eval;
pub mod syntax;
pub mod util;
