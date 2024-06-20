use crate::{
    ast::Expr,
    env::{Env, EnvExt},
    eval::eval_args,
    LangError, LangResult,
};
use std::{collections::HashMap, convert::identity, rc::Rc};

pub fn trim_bracket_outer(s: &str) -> &str {
    fn trim_recursive(s: &str, right_pos: usize) -> &str {
        if s.starts_with("((") && s.ends_with("))") {
            trim_recursive(&s[1..right_pos], right_pos - 1)
        } else {
            s
        }
    }
    trim_recursive(s, s.len() - 1)
}

pub fn ensure<F>(args: &[Expr], body: F) -> LangResult<Expr>
where
    F: Fn(&&Expr, &&Expr) -> bool,
{
    let result = args.iter().map_windows(|[x, y]| body(x, y)).all(identity);
    Ok(Expr::Bool(result))
}

pub fn parse_list_of_strings(args: &[Expr]) -> LangResult<Vec<String>> {
    args.iter()
        .map(|expr| match expr {
            Expr::String(s) => Ok(s.clone()),
            _ => Err(LangError::Other("expected a string".into())),
        })
        .collect()
}

pub fn parse_list_of_floats(args: &[Expr]) -> LangResult<Vec<f64>> {
    args.iter()
        .map(|expr| match expr {
            Expr::Num(num) => Ok(*num),
            _ => Err(LangError::Other("expected a number".into())),
        })
        .collect()
}

// Every argument should be `Expr::Symbol`
pub fn parse_list_of_args(args: Rc<Expr>) -> LangResult<Vec<String>> {
    match args.as_ref() {
        Expr::Symbol(s) => Ok(vec![s.clone()]),
        Expr::SExpr(args) => args
            .iter()
            .map(|expr| match expr {
                Expr::Symbol(s) => Ok(s.clone()),
                _ => Err(LangError::Other("expected a symbol".into())),
            })
            .collect(),
        _ => Err(LangError::Unknown),
    }
}

pub fn child_env_for_lambda(params: Rc<Expr>, args: &[Expr], parent_env: Env) -> LangResult<Env> {
    let params = parse_list_of_args(params)?;
    if params.len() != args.len() {
        return Err(LangError::InvalidArgsLen);
    }

    let args = eval_args(args, parent_env.clone())?;
    let local = params.into_iter().zip(args);
    let local = HashMap::from_iter(local);

    let env = Env::new_env(local, Some(parent_env));
    Ok(env)
}
