extern crate lisp;

use lisp::syntax::codemap::Source;
use lisp::syntax::parse;
use lisp::syntax::pp;

fn eq(source: &str, expected_repr: &str) {
    let source = Source::new(source);
    let expr = parse::expr(source).unwrap();
    let repr = pp::expr(&expr, source);

    assert_eq!(repr, expected_repr)
}

#[test]
fn nil_true_false() {
    eq("nil", "nil");
    eq("true", "true");
    eq("false", "false");
}

#[test]
fn numbers() {
    eq("1", "1");
    eq("  7   ", "7");
}

#[test]
fn symbols() {
    eq("+", "+");
    eq("abc", "abc");
    eq("   abc   ", "abc");
    eq("abc5", "abc5");
    eq("abc-def", "abc-def");
}

#[test]
fn string() {
    eq("\"abc\"", "\"abc\"");
    eq("   \"abc\"   ", "\"abc\"");
    eq("\"abc (with parens)\"", "\"abc (with parens)\"");
    // TODO handle escaping
    //eq(r#""abc\"def""#, "");
    eq(r#""abc\ndef""#, "\"abc\\ndef\"");
}

#[test]
fn lists() {
    eq("(+ 1 2)", "(+ 1 2)");
    eq("((3 4))", "((3 4))");
    eq("(+ 1 (+ 2 3))", "(+ 1 (+ 2 3))");
    eq("  ( +   1   (+   2 3   )   )  ", "(+ 1 (+ 2 3))");
    eq("(* 1 2)", "(* 1 2)");
    eq("(** 1 2)", "(** 1 2)");
}

#[test]
fn commas() {
    eq("(1 2, 3,,,,),,", "(1 2 3)");
}

// TODO implement quoting
#[test]
#[ignore]
fn quoting() {
    eq(("'1"), "(quote 1)");
    eq(("'(1 2 3)"), "(quote (1 2 3))");
    eq(("`1"), "(quasiquote 1)");
    eq(("`(1 2 3)"), "(quasiquote (1 2 3))");
    eq(("~1"), "(unquote 1)");
    eq(("~(1 2 3)"), "(unquote (1 2 3))");
    eq(("~@(1 2 3)"), "(splice-unquote (1 2 3))");
}

#[test]
fn errors() {
    assert!(parse::expr(Source::new("(1 2")).is_err());
    assert!(parse::expr(Source::new("[1 2")).is_err());
    assert!(parse::expr(Source::new("\"abc")).is_err());
}

// TODO optional tests
