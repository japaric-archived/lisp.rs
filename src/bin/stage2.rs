#![feature(old_io)]

extern crate lisp;

use std::old_io::stdio;

use lisp::ast::Expr;
use lisp::env::Env;
use lisp::eval::{Value, self};
use lisp::parse::{Error, self};

fn read<'a>(input: &'a str) -> Result<Expr, Error<'a>> {
    parse::expr(input)
}

fn eval(expr: Expr, env: &Env) -> Result<Value, eval::Error> {
    eval::expr(&expr, env)
}

fn print(val: Value) {
    println!("{}", val)
}

fn main() {
    let mut stdin = stdio::stdin();
    let env = Env::default();

    print!("> ");
    for line in stdin.lock().lines() {
        if let Ok(line) = line {
            match read(&line) {
                Err(e) => println!("error: {:?}", e),
                Ok(expr) => match eval(expr, &env) {
                    Err(e) => println!("error: {:?}", e),
                    Ok(val) => print(val),
                },
            }
        } else {
            return;
        }

        print!("> ");
    }
}
