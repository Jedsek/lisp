#![allow(unused)]

use anyhow::Result;
use lang::{ast, LangParser, LangResult, Rule};
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

    // for expr in ast {
    //     let result = ast::eval_expr(&expr);
    //     match result {
    //         Ok(result) => println!("{}", result),
    //         Err(err) => eprintln!("{}", err),
    //     }
    // }

    Ok(())
}
