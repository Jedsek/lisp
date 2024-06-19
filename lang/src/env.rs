#![allow(unused)]

use anyhow::ensure;

use crate::{ast::Expr, LangError, LangResult};
use std::{cell::RefCell, collections::HashMap, convert::identity, rc::Rc};

type SymbolName = String;

#[derive(Debug)]
pub struct Env {
    parent: Option<Rc<RefCell<Env>>>,
    local: HashMap<SymbolName, Expr>,
}

pub trait Lambda {
    fn apply(env: &mut Env, args: Vec<Expr>) -> Option<f64>
    where
        Self: Sized;
}

fn parse_single_float(exp: &Expr) -> LangResult<f64> {
    match exp {
        Expr::Num(num) => Ok(*num),
        _ => Err(LangError::Other("expected a number".into())),
    }
}
fn parse_list_of_floats(args: &[Expr]) -> LangResult<Vec<f64>> {
    args.iter().map(parse_single_float).collect()
}

pub fn eval(expr: &Expr, env: &mut Env) -> LangResult<Expr> {
    match expr {
        Expr::Symbol(s) => env
            .local
            .get(s)
            .ok_or(LangError::InvalidSymbol(s.to_string()))
            .cloned(),
        Expr::Num(_) | Expr::Bool(_) => Ok(expr.clone()),
        Expr::SExpr(s_expr) => {
            let first = s_expr.first().unwrap();
            let args = &s_expr[1..];
            match eval(first, env)? {
                Expr::Fn(f) => {
                    let args = args
                        .iter()
                        .map(|x| eval(x, env))
                        .collect::<LangResult<Vec<Expr>>>()?;
                    f(&args)
                }
                _ => unimplemented!(),
            }
        }

        _ => unimplemented!(),
    }
}

fn ensure<F>(args: &[Expr], f: F) -> bool
where
    F: Fn(&&f64, &&f64) -> bool,
{
    let args = parse_list_of_floats(args).unwrap();
    let result = args.iter().map_windows(|[x, y]| f(x, y)).all(identity);
    result
}

impl Default for Env {
    fn default() -> Self {
        let mut env = Self {
            parent: None,
            local: HashMap::new(),
        };
        env.define(
            "+".into(),
            Expr::Fn(|args| {
                let sum = parse_list_of_floats(args)?
                    .iter()
                    .fold(0.0, |acc, e| acc + e);
                Ok(Expr::Num(sum))
            }),
        );
        env.define(
            "-".into(),
            Expr::Fn(|args| {
                let first = parse_single_float(&args[0])?;
                let sum_rest = parse_list_of_floats(&args[1..])?
                    .iter()
                    .fold(0.0, |acc, e| acc + e);
                Ok(Expr::Num(first - sum_rest))
            }),
        );
        env.define(
            "=".into(),
            Expr::Fn(|args| {
                let result = ensure(args, |x, y| x == y);
                Ok(Expr::Bool(result))
            }),
        );

        env.define(
            ">".into(),
            Expr::Fn(|args| {
                let result = ensure(args, |x, y| x > y);
                Ok(Expr::Bool(result))
            }),
        );
        env.define(
            ">=".into(),
            Expr::Fn(|args| {
                let result = ensure(args, |x, y| x >= y);
                Ok(Expr::Bool(result))
            }),
        );
        env.define(
            "<".into(),
            Expr::Fn(|args| {
                let result = ensure(args, |x, y| x < y);
                Ok(Expr::Bool(result))
            }),
        );
        env.define(
            "<=".into(),
            Expr::Fn(|args| {
                let result = ensure(args, |x, y| x <= y);
                Ok(Expr::Bool(result))
            }),
        );
        env
    }
}

impl Env {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn extend(parent: Rc<RefCell<Env>>) -> Self {
        Self {
            parent: Some(parent),
            ..Self::new()
        }
    }

    pub fn define(&mut self, symbol: SymbolName, lambda: Expr) -> LangResult<()> {
        self.local.insert(symbol, lambda);
        Ok(())
    }

    pub fn undefine(&mut self, symbol: SymbolName) {
        if self.local.contains_key(&symbol) {
            self.local.remove(&symbol);
        } else if let Some(parent) = &self.parent {
            parent.borrow_mut().undefine(symbol);
        }
    }
    pub fn get(&mut self, symbol: SymbolName) -> Option<Expr> {
        self.local.get(&symbol).cloned()
    }
}
