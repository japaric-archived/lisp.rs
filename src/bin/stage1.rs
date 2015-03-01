#![feature(old_io)]

extern crate lisp;

use std::old_io::stdio;

use lisp::ast::Expr;
use lisp::parse::{Error, self};

fn read<'a>(input: &'a str) -> Result<Expr, Error<'a>> {
    parse::expr(input)
}

fn eval(input: Expr) -> Expr {
    input
}

fn print(output: Expr) {
    println!("{}", output)
}

fn main() {
    let mut stdin = stdio::stdin();

    print!("> ");
    for line in stdin.lock().lines() {
        if let Ok(line) = line {
            match read(&line) {
                Ok(expr) => print(eval(expr)),
                Err(e) => println!("error: {:?}", e),
            }
        } else {
            return;
        }

        print!("> ");
    }
}
