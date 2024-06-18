mod ast;

use anyhow::Result;
use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct LangParser;

pub fn eval_expr(input: &str, _debug_mode: bool) -> Result<f64> {
    let parsed = LangParser::parse(Rule::program, input)?.next().unwrap();
    let ast = ast::from(parsed)?;
    let expr = ast.first().unwrap();
    crate::ast::eval_expr(expr)
}

pub fn debug(parsed_exprs: Pair<Rule>) {
    for expr in parsed_exprs.into_inner() {
        println!("{:?}:\n {}\n", expr.clone(), expr.as_str());
        // println!("{}\n", expr.as_str());
    }
}
