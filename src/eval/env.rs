//! Environment

use std::collections::HashMap;

use eval::Value;

/// Environment
pub type Env = HashMap<String, Value>;

/// The default environment
pub fn default() -> Env {
    let mut env = Env::new();

    env.insert(String::from_str("*"), Value::Function(mul));
    env.insert(String::from_str("+"), Value::Function(add));
    env.insert(String::from_str("-"), Value::Function(sub));
    env.insert(String::from_str("/"), Value::Function(div));
    env.insert(String::from_str("<"), Value::Function(lt));
    env.insert(String::from_str("<="), Value::Function(le));
    env.insert(String::from_str(">"), Value::Function(gt));
    env.insert(String::from_str(">="), Value::Function(ge));

    env
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
