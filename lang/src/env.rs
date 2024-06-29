use crate::{ast::Expr, builtin};
use indexmap::IndexMap;
use lru::LruCache;
use std::{cell::RefCell, num::NonZeroUsize, rc::Rc};

const CACHE_NUM: usize = 20;

pub type SymbolName = String;
pub type Map<K, V> = IndexMap<K, V>;

#[derive(Debug, Clone)]
pub struct Env {
    parent: Option<Rc<RefCell<Env>>>,
    local: Map<SymbolName, Expr>,
    pub cache: Rc<RefCell<LruCache<SymbolName, Expr>>>,
}

impl Default for Env {
    fn default() -> Self {
        let cache = LruCache::new(NonZeroUsize::new(CACHE_NUM).unwrap());
        let cache = Rc::new(RefCell::new(cache));
        let cache = cache;
        let mut env = Self {
            parent: None,
            local: Map::new(),
            cache,
        };
        builtin::define_std(&mut env);
        env
    }
}

impl Env {
    pub fn new(
        local: Map<SymbolName, Expr>,
        parent: Option<Rc<RefCell<Env>>>,
        cache: Rc<RefCell<LruCache<SymbolName, Expr>>>,
    ) -> Self {
        Self {
            local,
            parent,
            cache,
        }
    }

    // pub fn extend(parent: Env) -> Self {
    //     Self {
    //         parent: Some(parent),
    //         ..Self::default()
    //     }
    // }

    pub fn define(&mut self, symbol: SymbolName, value: Expr) {
        let mut cache = self.cache.borrow_mut();
        cache.put(symbol.clone(), value.clone());
        self.local.insert(symbol, value);
        // }
    }

    pub fn undefine(&mut self, symbol: &SymbolName) {
        if self.local.contains_key(symbol) {
            self.local.shift_remove(symbol);
        } else if let Some(parent) = &self.parent {
            parent.borrow_mut().undefine(symbol);
        }
    }
    pub fn get(&self, symbol: &SymbolName) -> Option<Expr> {
        if let Some(expr) = self.cache.borrow_mut().get(symbol) {
            return Some(expr.clone());
        }

        match self.local.get(symbol) {
            Some(value) => Some(value.clone()),
            None => match &self.parent {
                None => None,
                Some(parent) => parent.borrow_mut().get(symbol),
            },
        }
    }
}
