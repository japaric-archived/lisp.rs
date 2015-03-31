extern crate lisp;

mod eval;

use lisp::eval::env;
use lisp::syntax::ast::interner::Interner;

#[test]
fn env() {
    let ref mut interner = Interner::new();
    let ref mut env = env::default(interner);

    eval::eq("(+ 1 2)", "3", env, interner);
    eval::eq("(/ (- (+ 5 (* 2 3)) 3) 4)", "2", env, interner);
}

#[test]
fn def() {
    let ref mut interner = Interner::new();
    let ref mut env = env::default(interner);

    eval::eq("(def! x 3)", "3", env, interner);
    eval::eq("(def! x 4)", "4", env, interner);
    eval::eq("x", "4", env, interner);
    eval::eq("(def! y (+ 1 7))", "8", env, interner);
    eval::eq("y", "8", env, interner);
}

// TODO implement `let*`
#[ignore]
#[test]
fn let_() {
    let ref mut interner = Interner::new();
    let ref mut env = env::default(interner);

    eval::eq("(let* (z 9) z)", "9", env, interner);
    eval::eq("(let* (x 9) x)", "9", env, interner);
    eval::eq("x", "4", env, interner);
    eval::eq("(let* (z (+ 2 3)) (+ 1 z))", "6", env, interner);
    eval::eq("(let* (p (+ 2 3) q (+ 2 p)) (+ p q))", "12", env, interner);
}

#[ignore]
#[test]
fn outer() {
    let ref mut interner = Interner::new();
    let ref mut env = env::default(interner);

    eval::eq("(def! a 4)", "4", env, interner);
    eval::eq("(let* (q 9) q)", "9", env, interner);
    eval::eq("(let* (q 9) a)", "4", env, interner);
    eval::eq("(let* (z 2) (let* (q 9) a))", "4", env, interner);
}
