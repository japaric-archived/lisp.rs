//! Environment

use std::collections::HashMap;

use eval::{Function, Value};

/// Environment
pub struct Env(HashMap<String, Function>);

fn add(args: &[Value]) -> Option<Value> {
    match args {
        [Value::Integer(a), Value::Integer(b)] => Some(Value::Integer(a + b)),
        _ => None,
    }
}

fn div(args: &[Value]) -> Option<Value> {
    match args {
        [Value::Integer(a), Value::Integer(b)] => Some(Value::Integer(a / b)),
        _ => None,
    }
}

fn mul(args: &[Value]) -> Option<Value> {
    match args {
        [Value::Integer(a), Value::Integer(b)] => Some(Value::Integer(a * b)),
        _ => None,
    }
}

fn sub(args: &[Value]) -> Option<Value> {
    match args {
        [Value::Integer(a), Value::Integer(b)] => Some(Value::Integer(a - b)),
        _ => None,
    }
}

impl Env {
    /// A new empty environment
    fn new() -> Env {
        Env(HashMap::new())
    }

    /// The default environment
    pub fn default() -> Env {
        let mut env = Env::new();

        env.0.insert(String::from_str("*"), Box::new(mul));
        env.0.insert(String::from_str("+"), Box::new(add));
        env.0.insert(String::from_str("-"), Box::new(sub));
        env.0.insert(String::from_str("/"), Box::new(div));

        env
    }

    /// Checks if `symbol` has been defined in this environment
    pub fn contains(&self, symbol: &str) -> bool {
        self.0.contains_key(symbol)
    }

    /// Retrieves the function associated to `symbol`, if any
    pub fn get(&self, symbol: &str) -> Option<&Function> {
        self.0.get(symbol)
    }
}
