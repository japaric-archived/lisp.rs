#![allow(dead_code)]

use lisp::eval::env::Stack;
use lisp::eval;
use lisp::syntax::codemap::Source;
use lisp::syntax::parse;
use lisp::util::interner::Interner;

pub use lisp::eval::Error_;

pub fn eq(input: &str, output: &str, env: &mut Stack, interner: &mut Interner) {
    let source = Source::new(input);

    let ast = parse::expr(source, interner).unwrap();

    assert_eq!(output, eval::expr(&ast, source, env).unwrap().display(interner))
}

pub fn err(input: &str, error: eval::Error_, env: &mut Stack, interner: &mut Interner) {
    let source = Source::new(input);

    assert_eq!{
        error,
        eval::expr(&parse::expr(source, interner).unwrap(), source, env).unwrap_err().node
    }
}
