extern crate lisp;

mod eval;

use lisp::eval::env;
use lisp::syntax::ast::interner::Interner;

#[test]
fn eq() {
    let ref mut interner = Interner::new();
    let ref mut env = env::default(interner);

    eval::eq("(+ 1 2)", "3", env, interner);
    eval::eq("(+ 5 (* 2 3))", "11", env, interner);
    eval::eq("(- (+ 5 (* 2 3)) 3)", "8", env, interner);
    eval::eq("(/ (- (+ 5 (* 2 3)) 3) 4)", "2", env, interner);
    eval::eq("(/ (- (+ 515 (* 222 311)) 302) 27)", "2565", env, interner);
}

#[test]
fn err() {
    let ref mut interner = Interner::new();
    let ref mut env = env::default(interner);

    eval::err("(abc 1 2 3)", eval::Error_::UndefinedSymbol, env, interner);
}
