//! Environment

use std::collections::HashMap;

use eval::Value;
use util::interner::{Interner, Name};

/// A stack of environments
pub struct Stack<'a> {
    top: Env,
    bottom: Option<&'a Stack<'a>>,
}

impl<'a> Stack<'a> {
    /// Pushes a new environment into the stack
    pub fn push(&self, env: Env) -> Stack {
        Stack {
            top: env,
            bottom: Some(self),
        }
    }

    /// Searches the stack (from top to bottom) and retrieves the first value that's associated to
    /// `symbol`
    pub fn get(&self, symbol: &Name) -> Option<&Value> {
        self.top.get(symbol).or_else(|| {
            self.bottom.as_ref().and_then(|stack| stack.get(symbol))
        })
    }

    /// Inserts a `symbol`/`value` pair in the top environment
    pub fn insert(&mut self, symbol: Name, value: Value) {
        self.top.insert(symbol, value);
    }
}

/// Environment
pub type Env = HashMap<Name, Value>;

/// The default environment stack
pub fn default(interner: &mut Interner) -> Stack<'static> {
    let mut env = Env::new();

    env.insert(interner.intern("*"), Value::Function(mul));
    env.insert(interner.intern("+"), Value::Function(add));
    env.insert(interner.intern("-"), Value::Function(sub));
    env.insert(interner.intern("/"), Value::Function(div));
    env.insert(interner.intern("<"), Value::Function(lt));
    env.insert(interner.intern("<="), Value::Function(le));
    env.insert(interner.intern(">"), Value::Function(gt));
    env.insert(interner.intern(">="), Value::Function(ge));

    Stack {
        top: env,
        bottom: None,
    }
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
