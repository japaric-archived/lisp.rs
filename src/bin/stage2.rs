#![feature(exit_status)]
#![feature(io)]

extern crate lines;
extern crate lisp;

use std::env;
use std::io::{StdoutLock, Write, self};

use lines::Lines;
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

fn print(val: Value, stdout: &mut StdoutLock) -> io::Result<()> {
    stdout.write_fmt(format_args!("{}\n", val))
}

fn rep(stdout: &mut StdoutLock) -> io::Result<()> {
    const PROMPT: &'static str = "> ";

    let stdin = io::stdin();
    let mut lines = Lines::from(stdin.lock());
    let env = Env::default();

    try!(stdout.write_all(PROMPT.as_bytes()));
    try!(stdout.flush());
    while let Some(line) = lines.next() {
        match read(try!(line)) {
            Err(e) => try!(stdout.write_fmt(format_args!("error: {:?}\n", e))),
            Ok(expr) => match eval(expr, &env) {
                Err(e) => try!(stdout.write_fmt(format_args!("error: {:?}\n", e))),
                Ok(val) => try!(print(val, stdout)),
            }
        }

        try!(stdout.write_all(PROMPT.as_bytes()));
        try!(stdout.flush());
    }

    Ok(())
}

fn main() {
    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    if let Err(e) = rep(&mut stdout) {
        env::set_exit_status(1);
        stdout.write_fmt(format_args!("{}", e)).ok();
    }
}
