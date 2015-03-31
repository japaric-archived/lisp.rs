//! Interner

use std::collections::HashMap;

use syntax::ast::Symbol;

/// A symbol interner
pub struct Interner {
    count: usize,
    map: HashMap<String, Symbol>,
}

impl Interner {
    /// A new empty interner
    pub fn new() -> Interner {
        Interner {
            count: 0,
            map: HashMap::new(),
        }
    }

    /// Interns `string` as a symbol, and returns the interned symbol
    pub fn intern(&mut self, string: &str) -> Symbol {
        if let Some(symbol) = self.map.get(string) {
            return symbol.clone();
        }

        let symbol = Symbol(self.count);

        self.count += 1;
        self.map.insert(String::from_str(string), symbol);

        symbol
    }
}
