extern crate lisp;

mod eval;

use lisp::eval::env;

#[test]
fn eq() {
    let ref mut env = env::default();

    eval::eq("(+ 1 2)", "3", env);
    eval::eq("(+ 5 (* 2 3))", "11", env);
    eval::eq("(- (+ 5 (* 2 3)) 3)", "8", env);
    eval::eq("(/ (- (+ 5 (* 2 3)) 3) 4)", "2", env);
    eval::eq("(/ (- (+ 515 (* 222 311)) 302) 27)", "2565", env);
}

#[test]
fn err() {
    eval::err("(abc 1 2 3)", eval::Error_::UndefinedSymbol);
}
