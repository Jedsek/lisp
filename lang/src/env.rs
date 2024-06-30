use crate::{ast::Expr, builtin};
use lru::LruCache;
use std::{cell::RefCell, collections::BTreeMap, num::NonZeroUsize, rc::Rc};

const CACHE_NUM: usize = 20;

pub type SymbolName = String;
pub type Map = BTreeMap<SymbolName, Expr>;

// NOTE/FIXME
// The Enviroment/Nested-Scope implement has problem now.
// The cache here is just for improvement performance in recursive lambda.
//
// Future code:
//
/*
    #[derive(Debug, Clone)]
    pub struct Env {
        current_scope: usize,
        local: Vec<Map>,
        // local_cache: LruCache   // <-- ?
    }

    pub fn get(&self, key: &SymbolName) -> Option<Expr> {
        get_inner(self.current_scope, key)
    }

    fn get_inner(&self, scope: usize, key: &SymbolName) -> Option<Expr> {
        let local = &self.local[scope];
        if let Some(expr) = local.get(key) {
            Some(expr)
        } else if scope > 0 {
            get_inner(scope - 1, key)
        } else {
            None
        }
    }

    pub define() { /*  */ }
    pub undefine() {/* */ }
    pub extend() { /* */ }
*/

// Shit code here, but I have no time to fix :(
// I will fix it in next year(2026)
#[derive(Debug, Clone)]
pub struct Env {
    parent: Option<Rc<RefCell<Env>>>,
    local: Map,
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
        local: Map,
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
            self.local.remove(symbol);
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
