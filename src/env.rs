//! Environment

use std::borrow::{Cow, IntoCow};
use std::collections::HashMap;

use eval::{Error, Value};

/// Environment
pub struct Env(HashMap<Cow<'static, str>, Box<Fn(&[Value]) -> Result<Value, Error>>>);

fn add(args: &[Value]) -> Result<Value, Error> {
    match args {
        [Value::Integer(a), Value::Integer(b)] => Ok(Value::Integer(a + b)),
        _ => Err(Error::UnsupportedOperation),
    }
}

fn div(args: &[Value]) -> Result<Value, Error> {
    match args {
        [Value::Integer(a), Value::Integer(b)] => Ok(Value::Integer(a / b)),
        _ => Err(Error::UnsupportedOperation),
    }
}

fn mul(args: &[Value]) -> Result<Value, Error> {
    match args {
        [Value::Integer(a), Value::Integer(b)] => Ok(Value::Integer(a * b)),
        _ => Err(Error::UnsupportedOperation),
    }
}

fn sub(args: &[Value]) -> Result<Value, Error> {
    match args {
        [Value::Integer(a), Value::Integer(b)] => Ok(Value::Integer(a - b)),
        _ => Err(Error::UnsupportedOperation),
    }
}

impl Env {
    /// Creates the default environment
    pub fn default() -> Env {
        let mut env = Env::new();

        env.0.insert("+".into_cow(), Box::new(add));
        env.0.insert("/".into_cow(), Box::new(div));
        env.0.insert("*".into_cow(), Box::new(mul));
        env.0.insert("-".into_cow(), Box::new(sub));

        env
    }

    fn new() -> Env {
        Env(HashMap::new())
    }

    /// Retrieves a function
    pub fn get(&self, sym: &str) -> Option<&Box<Fn(&[Value]) -> Result<Value, Error>>> {
        self.0.get(sym)
    }
}
