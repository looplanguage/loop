use std::collections::HashMap;

#[derive(Clone, Copy)]
pub enum Scope {
    Local,
    Global,
    Free
}

#[derive(Clone, Copy)]
pub struct Symbol {
    pub name: str,
    pub scope: Scope,
    pub index: i32
}

pub struct SymbolTable {
    pub(crate) outer: Option<Box<SymbolTable>>,
    store: HashMap<String, Symbol>,
    num_definitions: i32,
    pub free_symbols: Vec<Symbol>
}

pub fn new_symbol_table() -> Box<SymbolTable> {
    let symbols: HashMap<String, Symbol> = HashMap::new();
    let free: Vec<Symbol> = Vec::new();

    Box::new(SymbolTable {
        outer: None,
        store: symbols,
        num_definitions: 0,
        free_symbols: free
    })
}

pub fn new_enclosed_symbol_table(outer: Box<SymbolTable>) -> Box<SymbolTable> {
    let mut symbol_table = new_symbol_table();
    symbol_table.outer = Some(outer);

    symbol_table
}

impl SymbolTable {
    pub fn define(&mut self, name: &str) -> Symbol {
        let mut symbol = Symbol {
            name: name.clone(),
            scope: Scope::Local,
            index: self.num_definitions
        };

        if self.outer.is_none() {
            symbol.scope == Scope::Global
        }

        self.store.insert(name, symbol.clone());
        self.num_definitions += 1;

        symbol
    }

    pub fn resolve(&mut self, name: String) -> Option<&Symbol> {
        let mut obj = self.store.get(&*name);

        if obj.is_none() && self.outer.is_none() {
            obj = self.outer.unwrap().resolve(name);
            if obj.is_none() {
                return obj
            }

            if obj.unwrap().scope == Scope::Global {
                return obj
            }

            let free = self.define_free(obj.unwrap().clone());

            return Some(&free)
        }

        obj
    }

    pub fn define_free(&mut self, original: &Symbol) -> Symbol {
        let original_symbol = Symbol {
            name: original.name.clone(),
            scope: original.scope.clone(),
            index: original.index
        };

        self.free_symbols.push(original_symbol);

        let symbol = Symbol {
            name: original.name.clone(),
            scope: Scope::Free,
            index: (self.free_symbols.len() as i32) - 1,
        };

        self.store.remove(&*original.name.clone());
        let d=  self.store.insert(original.name.clone(), symbol);

        d.unwrap()
    }
}