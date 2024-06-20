use std::rc::Rc;

use crate::{
    ast::{Expr, Lambda},
    env::{Env, EnvExt},
    utils::child_env_for_lambda,
    LangError, LangResult,
};

pub fn eval(expr: &Expr, env: Env) -> LangResult<Expr> {
    match expr {
        Expr::Symbol(s) => env.get(s.into()).ok_or(LangError::InvalidSymbol(s.into())),
        Expr::Num(_) | Expr::Bool(_) | Expr::String(_) | Expr::QExpr(_) => Ok(expr.clone()),
        Expr::SExpr(s_expr) => eval_sexpr(s_expr, env.clone()),
        _ => unimplemented!(),
    }
}

pub fn eval_sexpr(s_expr: &[Expr], env: Env) -> LangResult<Expr> {
    let len = s_expr.len();
    if len == 0 {
        return Ok(Expr::Nil);
    } else if len == 1 {
        return eval(&s_expr[0], env);
    }

    let first = s_expr.first().unwrap();
    let args = &s_expr[1..];
    if let Some(result) = eval_builtin(first, args, env.clone()) {
        return result;
    }
    let first_eval = eval(first, env.clone())?;
    match first_eval {
        Expr::Fn(f) => f(&eval_args(args, env.clone())?),
        Expr::Lambda(lambda) => {
            let child_env = child_env_for_lambda(lambda.params, args, env)?;
            eval(&lambda.body, child_env)
        }
        e => Err(LangError::InvalidSymbol(e.to_string())),
    }
}

pub fn eval_args(args: &[Expr], env: Env) -> LangResult<Vec<Expr>> {
    args.iter().map(|x| eval(x, env.clone())).collect()
}

fn eval_builtin(expr: &Expr, args: &[Expr], env: Env) -> Option<LangResult<Expr>> {
    match expr {
        Expr::Symbol(s) => match s.as_str() {
            "if" => Some(eval_if(args, env.clone())),
            "def" => Some(eval_def(args, env.clone())),
            "fn" => Some(eval_lambda(args, env.clone())),
            _ => None,
        },
        _ => None,
    }
}

fn eval_lambda(args: &[Expr], _env: Env) -> Result<Expr, LangError> {
    if args.len() != 2 {
        return Err(LangError::InvalidArgsLen);
    }

    Ok(Expr::Lambda(Lambda {
        params: Rc::new(args.first().unwrap().clone()),
        body: Rc::new(args.get(1).unwrap().clone()),
    }))
}

fn eval_if(args: &[Expr], env: Env) -> LangResult<Expr> {
    if args.len() != 3 {
        return Err(LangError::InvalidArgsLen);
    }

    let condition = &args[0];
    let condition = eval(condition, env.clone())?;
    let Expr::Bool(condition) = condition else {
        return Err(LangError::InvalidCondition);
    };

    let idx = if condition { 1 } else { 2 };
    let result = &args[idx];
    eval(result, env)
}

fn eval_def(args: &[Expr], env: Env) -> LangResult<Expr> {
    if args.len() != 2 {
        return Err(LangError::InvalidArgsLen);
    }

    let symbol = &args[0];
    let symbol_name = match symbol {
        Expr::Symbol(s) => Ok(s.clone()),
        _ => Err(LangError::InvalidSymbol(format!("{symbol}"))),
    }?;
    let value = &args[1];
    let value = eval(value, env.clone())?;
    env.define(symbol_name, value);
    Ok(symbol.clone())
}
