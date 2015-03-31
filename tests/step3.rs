extern crate lisp;

mod eval;

use lisp::eval::env;

#[test]
fn env() {
    let ref mut env = env::default();

    eval::eq("(+ 1 2)", "3", env);
    eval::eq("(/ (- (+ 5 (* 2 3)) 3) 4)", "2", env);
}

#[test]
fn def() {
    let ref mut env = env::default();

    eval::eq("(def! x 3)", "3", env);
    eval::eq("(def! x 4)", "4", env);
    eval::eq("x", "4", env);
    eval::eq("(def! y (+ 1 7))", "8", env);
    eval::eq("y", "8", env);
}

// TODO implement `let*`
#[ignore]
#[test]
fn let_() {
    let ref mut env = env::default();

    eval::eq("(let* (z 9) z)", "9", env);
    eval::eq("(let* (x 9) x)", "9", env);
    eval::eq("x", "4", env);
    eval::eq("(let* (z (+ 2 3)) (+ 1 z))", "6", env);
    eval::eq("(let* (p (+ 2 3) q (+ 2 p)) (+ p q))", "12", env);
}

#[ignore]
#[test]
fn outer() {
    let ref mut env = env::default();

    eval::eq("(def! a 4)", "4", env);
    eval::eq("(let* (q 9) q)", "9", env);
    eval::eq("(let* (q 9) a)", "4", env);
    eval::eq("(let* (z 2) (let* (q 9) a))", "4", env);
}
