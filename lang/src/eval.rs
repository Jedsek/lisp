use std::rc::Rc;

use crate::{
    ast::{Expr, Lambda},
    env::Env,
    utils::child_env_for_lambda,
    LangError, LangResult,
};

pub fn eval(expr: &Expr, env: &mut Env) -> LangResult<Expr> {
    match expr {
        Expr::Symbol(s) => env.get(s.into()).ok_or(LangError::InvalidSymbol(s.into())),
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
    // env.define("else".into(), Expr::Bool(true));

    let len = args.len();
    if len < 2 {
        return Err(LangError::InvalidArgsLen);
    }
    let conds = &args[..(len - 1)];
    for cond in conds {
        match cond {
            Expr::SExpr(s_expr) => {
                // println!("{s_expr:?}");
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

    let condition = eval(&args[0], env)?;
    let Expr::Bool(condition) = condition else {
        return Err(LangError::TypeMismatched);
    };

    let idx = if condition { 1 } else { 2 };
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
        let symbol_name = match symbol {
            Expr::Symbol(s) => Ok(s.clone()),
            _ => Err(LangError::InvalidSymbol(format!("{symbol}"))),
        }?;

        let params = &function[1..];
        let params = Expr::SExpr(params.to_vec());
        let body = args[1].clone();
        let lambda = &[params, body];
        let lambda = eval_lambda(lambda, env)?;
        env.define(symbol_name, lambda);
        return Ok(symbol.clone());
    }

    let symbol = &args[0];
    let symbol_name = match symbol {
        Expr::Symbol(s) => Ok(s.clone()),
        _ => Err(LangError::InvalidSymbol(format!("{symbol}"))),
    }?;
    let value = &args[1];
    let value = eval(value, env)?;
    env.define(symbol_name, value);
    Ok(symbol.clone())
}
