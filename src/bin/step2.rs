#![feature(exit_status)]

extern crate lines;
extern crate lisp;

use std::io::{StdoutLock, Write, self};

use lines::Lines;
use lisp::diagnostics;
use lisp::eval::env::{Stack, self};
use lisp::eval::{Value, self};
use lisp::syntax::ast::Expr;
use lisp::syntax::codemap::Source;
use lisp::syntax::{parse, self};
use lisp::util::interner::Interner;

fn read(source: &Source, interner: &mut Interner) -> Result<Expr, syntax::Error> {
    parse::expr(source, interner)
}

fn eval(input: &Expr, source: &Source, env: &mut Stack) -> Result<Value, eval::Error> {
    eval::expr(input, source, env)
}

fn print(value: &Value, interner: &Interner, stdout: &mut StdoutLock) -> io::Result<()> {
    writeln!(stdout, "{}", value.display(interner))
}

fn rep(stdout: &mut StdoutLock) -> io::Result<()> {
    const PROMPT: &'static str = "> ";

    let stdin = io::stdin();
    let mut lines = Lines::from(stdin.lock());

    let ref mut interner = Interner::new();
    let ref mut env = env::default(interner);

    try!(stdout.write_all(PROMPT.as_bytes()));
    try!(stdout.flush());
    while let Some(line) = lines.next() {
        let source = Source::new(try!(line));

        if !source.as_str().trim().is_empty() {
            match read(source, interner) {
                Err(error) => {
                    try!(stdout.write_all(diagnostics::syntax(error, source).as_bytes()))
                },
                Ok(expr) => match eval(&expr, source, env) {
                    Err(error) => {
                        try!(stdout.write_all(diagnostics::eval(error, source).as_bytes()))
                    },
                    Ok(value) => try!(print(&value, interner, stdout)),
                },
            }
        }

        try!(stdout.write_all(PROMPT.as_bytes()));
        try!(stdout.flush());
    }

    Ok(())
}

fn main() {
    use std::env;

    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    if let Err(e) = rep(&mut stdout) {
        env::set_exit_status(1);
        writeln!(&mut stdout, "{}", e).ok();
    }
}
