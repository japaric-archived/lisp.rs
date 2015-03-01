#![feature(old_io)]

use std::old_io::stdio;

fn read(input: &str) -> &str {
    input
}

fn eval(input: &str) -> &str {
    input
}

fn print(output: &str) {
    print!("{}", output)
}

fn main() {
    let mut stdin = stdio::stdin();

    print!("> ");
    for line in stdin.lock().lines() {
        if let Ok(line) = line {
            print(eval(read(&line)))
        } else {
            return;
        }

        print!("> ");
    }
}
