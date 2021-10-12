use std::collections::HashMap;
use std::mem;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Scope {
    Local,
    Global,
    Free,
    Builtin,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Symbol {
    pub scope: Scope,
    pub index: u32,
}

#[derive(Default)]
pub struct SymbolLayer {
    store: HashMap<String, Symbol>,
    num_definitions: u32,
    pub free_symbols: Vec<Symbol>,
}

impl SymbolLayer {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn define_free(&mut self, name: &str, original: Symbol) -> Symbol {
        let symbol = Symbol {
            index: self.free_symbols.len() as u32,
            scope: Scope::Free,
        };

        self.free_symbols.push(original);
        *self.define_symbol(name, symbol)
    }

    pub fn define_symbol(&mut self, name: &str, symbol: Symbol) -> &Symbol {
        self.store.insert(name.to_string(), symbol);
        self.store.get(name).expect("inserted")
    }
}

#[derive(Default)]
pub struct SymbolTable {
    current: SymbolLayer,
    outers: Vec<SymbolLayer>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn new_with_builtins() -> Self {
        SymbolTable::new()
    }

    pub fn push(&mut self) {
        let outer = mem::replace(&mut self.current, SymbolLayer::new());
        self.outers.push(outer);
    }

    pub fn pop(&mut self) -> Vec<Symbol> {
        match self.outers.pop() {
            None => vec![],
            Some(outer) => {
                let popped = mem::replace(&mut self.current, outer);
                popped.free_symbols
            }
        }
    }

    pub fn define(&mut self, name: &str, global_index: u32) -> &Symbol {
        let scope = if self.outers.is_empty() || global_index > 0 {
            Scope::Global
        } else {
            Scope::Local
        };

        let mut symbol = Symbol {
            index: self.current.num_definitions,
            scope,
        };

        if global_index > 0 {
            symbol.index = global_index;
        }

        self.current.num_definitions += 1;

        self.current.define_symbol(name, symbol)
    }

    pub fn resolve(&mut self, name: &str) -> Option<Symbol> {
        {
            // Silence the borrow checker.
            // https://users.rust-lang.org/t/solved-borrow-doesnt-drop-returning-this-value-requires-that/24182
            let maybe_symbol: Option<&Symbol> =
                unsafe { mem::transmute(self.current.store.get(name)) };
            if maybe_symbol.is_some() {
                return maybe_symbol.copied();
            }
        }

        let num_outers = self.outers.len();
        // Try from the 2nd innermost store to the outermost one.
        for (i, outer) in self.outers.iter().rev().enumerate() {
            if let Some(original) = outer.store.get(name) {
                return match original.scope {
                    Scope::Global | Scope::Builtin => Some(*original),
                    Scope::Local | Scope::Free => {
                        let mut parent_symbol = *original;
                        for j in (num_outers - i)..num_outers {
                            let o = &mut self.outers[j];
                            parent_symbol = o.define_free(name, parent_symbol);
                        }
                        Some(self.current.define_free(name, parent_symbol))
                    }
                };
            }
        }
        None
    }

    pub fn num_definitions(&self) -> u32 {
        self.current.num_definitions
    }
}
