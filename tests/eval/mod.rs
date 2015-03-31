#![allow(dead_code)]

use lisp::eval::env::Env;
use lisp::eval;
use lisp::syntax::codemap::Source;
use lisp::syntax::parse;

pub use lisp::eval::Error_;

pub fn eq(input: &str, output: &str, env: &mut Env) {
    let source = Source::new(input);

    let ast = parse::expr(source).unwrap();

    assert_eq!(output, eval::expr(&ast, source, env).unwrap().to_string())
}

pub fn err(input: &str, error: eval::Error_) {
    let source = Source::new(input);
    let mut env = Env::default();

    assert_eq!{
        error,
        eval::expr(&parse::expr(source).unwrap(), source, &mut env).unwrap_err().node
    }
}
