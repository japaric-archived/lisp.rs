#![feature(exit_status)]
#![feature(io)]

extern crate lines;

use lines::Lines;

use std::env;
use std::io::{StdoutLock, Write, self};

fn read(input: &str) -> &str {
    input
}

fn eval(input: &str) -> &str {
    input
}

fn print(output: &str, stdout: &mut StdoutLock) -> io::Result<()> {
    stdout.write_fmt(format_args!("{}\n", output))
}

fn rep(stdout: &mut StdoutLock) -> io::Result<()> {
    const PROMPT: &'static str = "> ";

    let stdin = io::stdin();
    let mut lines = Lines::from(stdin.lock());

    try!(stdout.write_all(PROMPT.as_bytes()));
    try!(stdout.flush());
    while let Some(line) = lines.next() {
        try!(print(eval(read(try!(line))), stdout));

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
