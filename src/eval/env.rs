//! Environment

use std::collections::HashMap;

use eval::Value;
use util::interner::{Interner, Name};

/// Environment
pub type Env = HashMap<Name, Value>;

/// The default environment
pub fn default(interner: &mut Interner) -> Env {
    let mut env = Env::new();

    env.insert(interner.intern("*"), Value::Function(mul));
    env.insert(interner.intern("+"), Value::Function(add));
    env.insert(interner.intern("-"), Value::Function(sub));
    env.insert(interner.intern("/"), Value::Function(div));
    env.insert(interner.intern("<"), Value::Function(lt));
    env.insert(interner.intern("<="), Value::Function(le));
    env.insert(interner.intern(">"), Value::Function(gt));
    env.insert(interner.intern(">="), Value::Function(ge));

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
