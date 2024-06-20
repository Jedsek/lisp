#![allow(unused)]

use anyhow::Result;
use lang::{
    ast,
    env::{Env, EnvExt},
    eval::eval,
    LangParser, LangResult, Rule,
};
use pest::Parser;
use std::fs;

fn main() -> LangResult<()> {
    let input = fs::read_to_string("test.lisp").expect("Failed to read file");
    let parsed_exprs = LangParser::parse(Rule::program, &input)
        .expect("Failed to parse")
        .next()
        .unwrap();

    // debug(parsed_exprs.clone());

    let ast = ast::from(parsed_exprs)?;
    let env = Env::default_env();

    for expr in ast {
        eval(&expr, env.clone());
    }

    Ok(())
}
