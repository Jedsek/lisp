use crate::{ast::Expr, builtin};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub type Env = Rc<RefCell<EnvInner>>;
pub type SymbolName = String;

#[derive(Debug, Clone)]
pub struct EnvInner {
    parent: Option<Env>,
    local: HashMap<SymbolName, Expr>,
}

#[allow(clippy::new_ret_no_self)]
pub trait EnvExt {
    fn default_env() -> Env {
        default()
    }

    fn new_env(local: HashMap<SymbolName, Expr>, parent: Option<Env>) -> Env {
        let env = EnvInner::new_with(local, parent);
        Rc::new(RefCell::new(env))
    }

    fn define(&self, symbol: SymbolName, value: Expr);
    fn undefine(&self, symbol: SymbolName);
    fn get(&self, symbol: SymbolName) -> Option<Expr>;
}

impl EnvExt for Env {
    fn define(&self, symbol: SymbolName, value: Expr) {
        self.borrow_mut().define(symbol, value);
    }

    fn undefine(&self, symbol: SymbolName) {
        self.borrow_mut().undefine(symbol);
    }

    fn get(&self, symbol: SymbolName) -> Option<Expr> {
        self.borrow().get(symbol)
    }
}

fn default() -> Env {
    let env = EnvInner {
        parent: None,
        local: HashMap::new(),
    };
    let env = Rc::new(RefCell::new(env));
    builtin::define_operaters(env.clone());
    env
}

impl EnvInner {
    pub fn new_with(local: HashMap<SymbolName, Expr>, parent: Option<Env>) -> Self {
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
            self.local.remove(&symbol);
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
