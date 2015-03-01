extern crate lisp;

use lisp::env::Env;
use lisp::eval;
use lisp::parse;

#[test]
fn eval() {
    let env = Env::default();

    let eval = |input| {
        match eval::expr(&parse::expr(input).unwrap(), &env) {
            Err(e) => format!("{:?}", e),
            Ok(val) => val.to_string(),
        }
    };

    assert_eq!(eval("(+ 1 2)"), "3");
    assert_eq!(eval("(+ 5 (* 2 3))"), "11");
    assert_eq!(eval("(- (+ 5 (* 2 3)) 3)"), "8");
    assert_eq!(eval("(/ (- (+ 5 (* 2 3)) 3) 4)"), "2");
    assert_eq!(eval("(/ (- (+ 515 (* 222 311)) 302) 27)"), "2565");
    assert_eq!(eval("(abc 1 2 3)"), "UndefinedSymbol(\"abc\")");
}
