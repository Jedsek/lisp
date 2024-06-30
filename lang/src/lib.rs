#![feature(iter_map_windows)]
#![feature(iterator_try_reduce)]

pub mod ast;
pub mod builtin;
pub mod codegen;
pub mod env;
pub mod eval;
pub mod utils;

use ast::Expr;
use pest::{iterators::Pair, Parser};
use pest_derive::Parser;
use thiserror::Error;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct LangParser;

#[derive(Error, Debug)]
pub enum LangError {
    #[error("Divided by zero")]
    DivideByZero,

    #[error("Failed to parse: {0}")]
    ParseFailed(String),

    #[error("Invalid symbol: {0}")]
    InvalidSymbol(String),

    #[error("Invalid length of arguments")]
    InvalidArgsLen,

    #[error("Type mismatched")]
    TypeMismatched,

    #[error("{0}")]
    Other(String),

    #[error("Unknown error")]
    Unknown,

    #[error("\nFuck you, not supported now. P!L!E!A!S!E! only calculate for it.\n
Below are possible solutions:

Sir/Madam, why do you:  
- use your precious fingers to type out these beautiful characters on the keyboard?
- waste time for executing your genius mind
- and get a \"Fuck you\" prompt?

Is this my fault as a developer for not implementing these features? 
No, no, no......It’s not like this, or rather, it shouldn’t be like this, because you must waste your precious time......Oh my God! 
It's like Uncle Tom kicked someone into a pit of fire.
It suddenly makes people realize that such an ugly and dirty lisp interpreter is not worth typing out these characters that go against fate.
You forced a naive, beautiful compiler that is still in its youth.
Just like ignoring the minor protection law of XXX state, destroying its vision and yearning for a better future.

oH MY GOD WHY THE FUCK DOES ANYONE REALLY READ ALL OF THIS Mister/Ms. YOU ARE FUCKING BORING AND I AM FUCKING BORING TOO FUCK IT!
")]
    FuckYou,
}

pub type LangResult<T> = Result<T, LangError>;

pub fn eval(input: &str) -> LangResult<Vec<Expr>> {
    let parsed = LangParser::parse(Rule::program, input)
        .map_err(|e| LangError::ParseFailed(format!("{e}")))?
        .next()
        .unwrap();
    let ast = ast::from(parsed)?;
    Ok(ast)
}

pub fn debug(parsed_exprs: Pair<Rule>) {
    for expr in parsed_exprs.into_inner() {
        println!("{:?}:\n {}\n", expr.clone(), expr.as_str());
        // println!("{}\n", expr.as_str());
    }
}
