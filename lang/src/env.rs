use crate::{ast::Expr, builtin};
use indexmap::IndexMap;
use std::{cell::RefCell, rc::Rc};

pub type SymbolName = String;
pub type Map<K, V> = IndexMap<K, V>;

#[derive(Debug, Clone)]
pub struct Env {
    parent: Option<Rc<RefCell<Env>>>,
    local: Map<SymbolName, Expr>,
}

impl Default for Env {
    fn default() -> Self {
        let mut env = Self {
            parent: None,
            local: Map::new(),
        };
        builtin::define_operaters(&mut env);
        env
    }
}

impl Env {
    pub fn new(local: Map<SymbolName, Expr>, parent: Option<Rc<RefCell<Env>>>) -> Self {
        Self { local, parent }
    }

    // pub fn extend(parent: Env) -> Self {
    //     Self {
    //         parent: Some(parent),
    //         ..Self::default()
    //     }
    // }

    pub fn define(&mut self, symbol: SymbolName, value: Expr) {
        self.local.insert(symbol, value);
    }

    pub fn undefine(&mut self, symbol: SymbolName) {
        if self.local.contains_key(&symbol) {
            self.local.shift_remove(&symbol);
        } else if let Some(parent) = &self.parent {
            parent.borrow_mut().undefine(symbol);
        }
    }
    pub fn get(&self, symbol: SymbolName) -> Option<Expr> {
        match self.local.get(&symbol) {
            Some(value) => Some(value.clone()),
            None => match &self.parent {
                None => None,
                Some(parent) => parent.borrow_mut().get(symbol),
            },
        }
    }
}
