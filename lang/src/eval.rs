use std::rc::Rc;

use crate::{
    ast::{Expr, Lambda},
    env::Env,
    utils::child_env_for_lambda,
    LangError, LangResult,
};

//
// FIXME:
// so many `clone()`.
// should improve performance.

pub fn eval(expr: &Expr, env: &mut Env) -> LangResult<Expr> {
    match expr {
        Expr::Symbol(s) => env.get(s).ok_or(LangError::InvalidSymbol(s.into())),
        Expr::Num(_) | Expr::Bool(_) | Expr::String(_) | Expr::Nil | Expr::QExpr(_) => {
            Ok(expr.clone())
        }
        Expr::SExpr(s_expr) => eval_sexpr(s_expr, env),
        _ => unimplemented!(),
    }
}

pub fn eval_sexpr(s_expr: &[Expr], env: &mut Env) -> LangResult<Expr> {
    let len = s_expr.len();

    if len == 0 {
        return Ok(Expr::Nil);
    }

    let first = s_expr.first().unwrap();
    let args = &s_expr[1..];

    if let Some(result) = eval_keyword(first, args, env) {
        return result;
    }
    let first_eval = eval(first, env)?;
    match first_eval {
        Expr::Fn(f) => f(&eval_args(args, env)?),
        Expr::Lambda(lambda) => {
            let mut child_env = child_env_for_lambda(lambda.params, args, env)?;
            eval(&lambda.body, &mut child_env)
        }
        expr if len == 1 => eval(&expr, env),
        expr => Err(LangError::InvalidSymbol(expr.to_string())),
    }
}

pub fn eval_args(args: &[Expr], env: &mut Env) -> LangResult<Vec<Expr>> {
    args.iter().map(|x| eval(x, env)).collect()
}

fn eval_keyword(expr: &Expr, args: &[Expr], env: &mut Env) -> Option<LangResult<Expr>> {
    match expr {
        Expr::Symbol(s) => match s.as_str() {
            "if" => Some(eval_if(args, env)),
            "cond" => Some(eval_cond(args, env)),
            "define" => Some(eval_def(args, env)),
            "lambda" => Some(eval_lambda(args, env)),
            _ => None,
        },
        _ => None,
    }
}

fn eval_cond(args: &[Expr], env: &mut Env) -> Result<Expr, LangError> {
    if args.len() < 2 {
        return Err(LangError::InvalidArgsLen);
    }
    for cond in args {
        match cond {
            Expr::SExpr(s_expr) => {
                // println!("{s_expr:?}");
                if s_expr.len() != 2 {
                    return Err(LangError::InvalidArgsLen);
                }
                if let Expr::Symbol(s) = &s_expr[0] {
                    if s == "else" {
                        return eval(&s_expr[1], env);
                    }
                }
                match eval(&s_expr[0], env)? {
                    Expr::Bool(true) => return eval(&s_expr[1], env),
                    Expr::Bool(false) => continue,
                    _ => return Err(LangError::TypeMismatched),
                }
            }
            _ => return Err(LangError::FuckYou),
        }
    }
    let other = args.last().unwrap();
    eval(other, env)
}

fn eval_lambda(args: &[Expr], _env: &Env) -> Result<Expr, LangError> {
    if args.len() != 2 {
        return Err(LangError::InvalidArgsLen);
    }

    Ok(Expr::Lambda(Lambda {
        params: Rc::new(args.first().unwrap().clone()),
        body: Rc::new(args.get(1).unwrap().clone()),
    }))
}

fn eval_if(args: &[Expr], env: &mut Env) -> LangResult<Expr> {
    if args.len() != 3 {
        return Err(LangError::InvalidArgsLen);
    }

    let cond = eval(&args[0], env)?;
    let cond = cond.inner_bool()?;

    let idx = if *cond { 1 } else { 2 };
    let result = &args[idx];
    eval(result, env)
}

fn eval_def(args: &[Expr], env: &mut Env) -> LangResult<Expr> {
    if args.len() != 2 {
        return Err(LangError::InvalidArgsLen);
    }

    if let Expr::SExpr(function) = &args[0] {
        if function.is_empty() {
            return Err(LangError::InvalidArgsLen);
        }

        let symbol = &function[0];
        let symbol_name = symbol.inner_symbol()?.clone();

        let params = &function[1..];
        let params = Expr::SExpr(params.to_vec());
        let body = args[1].clone();
        let lambda = &[params, body];
        let lambda = eval_lambda(lambda, env)?;
        env.define(symbol_name, lambda);
        return Ok(symbol.clone());
    }

    let symbol = &args[0].clone();
    let symbol_name = symbol.inner_symbol()?.clone();
    let value = eval(&args[1], env)?;
    env.define(symbol_name, value);
    Ok(symbol.clone())
}
