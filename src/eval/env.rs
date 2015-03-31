//! Environment

use std::collections::HashMap;

use eval::{Function, Value};

/// Environment
pub struct Env {
    /// Declared functions
    pub functions: HashMap<String, Function>,
    /// Declared variables
    pub variables: HashMap<String, Value>,
}

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

fn ge(args: &[Value]) -> Option<Value> {
    match args {
        [Value::Integer(a), Value::Integer(b)] => Some(Value::Bool(a >= b)),
        _ => None,
    }
}

fn gt(args: &[Value]) -> Option<Value> {
    match args {
        [Value::Integer(a), Value::Integer(b)] => Some(Value::Bool(a > b)),
        _ => None,
    }
}

fn le(args: &[Value]) -> Option<Value> {
    match args {
        [Value::Integer(a), Value::Integer(b)] => Some(Value::Bool(a <= b)),
        _ => None,
    }
}

fn lt(args: &[Value]) -> Option<Value> {
    match args {
        [Value::Integer(a), Value::Integer(b)] => Some(Value::Bool(a < b)),
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
        Env {
            functions: HashMap::new(),
            variables: HashMap::new(),
        }
    }

    /// The default environment
    pub fn default() -> Env {
        let mut env = Env::new();

        env.functions.insert(String::from_str("*"), mul);
        env.functions.insert(String::from_str("+"), add);
        env.functions.insert(String::from_str("-"), sub);
        env.functions.insert(String::from_str("/"), div);
        env.functions.insert(String::from_str("<"), lt);
        env.functions.insert(String::from_str("<="), le);
        env.functions.insert(String::from_str(">"), gt);
        env.functions.insert(String::from_str(">="), ge);

        env
    }
}
