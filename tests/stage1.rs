extern crate lisp;

use lisp::parse;

#[test]
fn nil_true_false() {
    assert_eq!(parse::expr("nil").unwrap().to_string(), "nil");
    assert_eq!(parse::expr("true").unwrap().to_string(), "true");
    assert_eq!(parse::expr("false").unwrap().to_string(), "false");
}

#[test]
fn numbers() {
    assert_eq!(parse::expr("1").unwrap().to_string(), "1");
    assert_eq!(parse::expr("7").unwrap().to_string(), "7");
    assert_eq!(parse::expr("  7   ").unwrap().to_string(), "7");
}

#[test]
fn symbols() {
    assert_eq!(parse::expr("+").unwrap().to_string(), "+");
    assert_eq!(parse::expr("abc").unwrap().to_string(), "abc");
    assert_eq!(parse::expr("   abc   ").unwrap().to_string(), "abc");
    assert_eq!(parse::expr("abc5").unwrap().to_string(), "abc5");
    assert_eq!(parse::expr("abc-def").unwrap().to_string(), "abc-def");
}

#[test]
fn string() {
    assert_eq!(parse::expr("\"abc\"").unwrap().to_string(), "\"abc\"");
    assert_eq!(parse::expr("   \"abc\"   ").unwrap().to_string(), "\"abc\"");
    assert_eq!(parse::expr("\"abc (with parens)\"").unwrap().to_string(), "\"abc (with parens)\"");
    // TODO handle escpaing
    //assert_eq!(parse::expr(r#""abc\"def""#).unwrap().to_string(), "");
    assert_eq!(parse::expr(r#""abc\ndef""#).unwrap().to_string(), "\"abc\\ndef\"");
}

#[test]
fn lists() {
    assert_eq!(parse::expr("(+ 1 2)").unwrap().to_string(), "(+ 1 2)");
    assert_eq!(parse::expr("((3 4))").unwrap().to_string(), "((3 4))");
    assert_eq!(parse::expr("(+ 1 (+ 2 3))").unwrap().to_string(), "(+ 1 (+ 2 3))");
    assert_eq!{
        parse::expr("  ( +   1   (+   2 3   )   )  ").unwrap().to_string(),
        "(+ 1 (+ 2 3))"
    };
    assert_eq!(parse::expr("(* 1 2)").unwrap().to_string(), "(* 1 2)");
    assert_eq!(parse::expr("(** 1 2)").unwrap().to_string(), "(** 1 2)");
}

#[test]
fn commas() {
    assert_eq!(parse::expr("(1 2, 3,,,,),,").unwrap().to_string(), "(1 2 3)");
}

// TODO implement quoting
#[test]
#[ignore]
fn quoting() {
    assert_eq!(parse::expr("'1").unwrap().to_string(), "(quote 1)");
    assert_eq!(parse::expr("'(1 2 3)").unwrap().to_string(), "(quote (1 2 3))");
    assert_eq!(parse::expr("`1").unwrap().to_string(), "(quasiquote 1)");
    assert_eq!(parse::expr("`(1 2 3)").unwrap().to_string(), "(quasiquote (1 2 3))");
    assert_eq!(parse::expr("~1").unwrap().to_string(), "(unquote 1)");
    assert_eq!(parse::expr("~(1 2 3)").unwrap().to_string(), "(unquote (1 2 3))");
    assert_eq!(parse::expr("~@(1 2 3)").unwrap().to_string(), "(splice-unquote (1 2 3))");
}

// TODO handle vectors
#[test]
fn errors() {
    assert!(parse::expr("(1 2").is_err());
    //assert!(parse::expr("[1 2").is_err());
    assert!(parse::expr("\"abc").is_err());
}

// TODO optional tests
