#![allow(unused)]

use anyhow::Result;
use lang::{ast, env::Env, eval::eval, LangParser, LangResult, Rule};
use pest::Parser;
use std::fs;

fn main() -> LangResult<()> {
    let path = std::env::args().nth(1).unwrap_or("test.lisp".into());
    let input = fs::read_to_string(path).expect("Failed to read file");

    let parsed_exprs = LangParser::parse(Rule::program, &input)
        .expect("Failed to parse")
        .next()
        .unwrap();

    // debug(parsed_exprs.clone());

    let ast = ast::from(parsed_exprs)?;
    let mut env = Env::default();

    for expr in ast {
        eval(&expr, &mut env)?;
    }

    Ok(())
}
