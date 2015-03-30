extern crate lisp;

use lisp::eval::env::Env;
use lisp::eval;
use lisp::syntax::codemap::Source;
use lisp::syntax::parse;

fn eq(input: &str, output: &str) {
    let source = Source::new(input);
    let env = Env::default();

    let ast = parse::expr(source).unwrap();

    assert_eq!(output, eval::expr(&ast, source, &env).unwrap().to_string())
}

fn err(input: &str, error: eval::Error_) {
    let source = Source::new(input);
    let env = Env::default();

    assert_eq!(error, eval::expr(&parse::expr(source).unwrap(), source, &env).unwrap_err().node)
}

#[test]
fn eval() {
    eq("(+ 1 2)", "3");
    eq("(+ 5 (* 2 3))", "11");
    eq("(- (+ 5 (* 2 3)) 3)", "8");
    eq("(/ (- (+ 5 (* 2 3)) 3) 4)", "2");
    eq("(/ (- (+ 515 (* 222 311)) 302) 27)", "2565");
    err("(abc 1 2 3)", eval::Error_::UndefinedSymbol);
}
