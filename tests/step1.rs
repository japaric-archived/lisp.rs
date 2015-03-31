extern crate lisp;

use lisp::syntax::ast::interner::Interner;
use lisp::syntax::codemap::Source;
use lisp::syntax::parse;
use lisp::syntax::pp;

fn eq(source: &str, expected_repr: &str, interner: &mut Interner) {
    let source = Source::new(source);
    let expr = parse::expr(source, interner).unwrap();
    let repr = pp::expr(&expr, source);

    assert_eq!(repr, expected_repr)
}

#[test]
fn nil_true_false() {
    let ref mut interner = Interner::new();

    eq("nil", "nil", interner);
    eq("true", "true", interner);
    eq("false", "false", interner);
}

#[test]
fn numbers() {
    let ref mut interner = Interner::new();

    eq("1", "1", interner);
    eq("  7   ", "7", interner);
}

#[test]
fn symbols() {
    let ref mut interner = Interner::new();

    eq("+", "+", interner);
    eq("abc", "abc", interner);
    eq("   abc   ", "abc", interner);
    eq("abc5", "abc5", interner);
    eq("abc-def", "abc-def", interner);
}

#[test]
fn string() {
    let ref mut interner = Interner::new();

    eq("\"abc\"", "\"abc\"", interner);
    eq("   \"abc\"   ", "\"abc\"", interner);
    eq("\"abc (with parens)\"", "\"abc (with parens)\"", interner);
    // TODO handle escaping
    //eq(r#""abc\"def""#, "", interner);
    eq(r#""abc\ndef""#, "\"abc\\ndef\"", interner);
}

#[test]
fn lists() {
    let ref mut interner = Interner::new();

    eq("(+ 1 2)", "(+ 1 2)", interner);
    eq("((3 4))", "((3 4))", interner);
    eq("(+ 1 (+ 2 3))", "(+ 1 (+ 2 3))", interner);
    eq("  ( +   1   (+   2 3   )   )  ", "(+ 1 (+ 2 3))", interner);
    eq("(* 1 2)", "(* 1 2)", interner);
    eq("(** 1 2)", "(** 1 2)", interner);
}

#[test]
fn commas() {
    let ref mut interner = Interner::new();

    eq("(1 2, 3,,,,),,", "(1 2 3)", interner);
}

// TODO implement quoting
#[test]
#[ignore]
fn quoting() {
    let ref mut interner = Interner::new();

    eq(("'1"), "(quote 1)", interner);
    eq(("'(1 2 3)"), "(quote (1 2 3))", interner);
    eq(("`1"), "(quasiquote 1)", interner);
    eq(("`(1 2 3)"), "(quasiquote (1 2 3))", interner);
    eq(("~1"), "(unquote 1)", interner);
    eq(("~(1 2 3)"), "(unquote (1 2 3))", interner);
    eq(("~@(1 2 3)"), "(splice-unquote (1 2 3))", interner);
}

#[test]
fn errors() {
    let ref mut interner = Interner::new();

    assert!(parse::expr(Source::new("(1 2"), interner).is_err());
    assert!(parse::expr(Source::new("[1 2"), interner).is_err());
    assert!(parse::expr(Source::new("\"abc"), interner).is_err());
}

// TODO optional tests
