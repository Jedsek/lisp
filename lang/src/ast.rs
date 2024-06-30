use std::fmt::Display;
use std::rc::Rc;

use crate::{LangError, LangParser, LangResult, Rule};
use pest::iterators::Pair;
use pest::Parser;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Expr {
    Nil,
    Num(f64),
    String(Rc<str>),
    Bool(bool),
    QExpr(Box<Expr>),
    SExpr(Vec<Expr>),
    Symbol(String),
    Fn(fn(&[Expr]) -> LangResult<Expr>),
    Lambda(Lambda),
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub enum ExprType {
    Nil,
    Num,
    String,
    Bool,
    QExpr,
    SExpr,
    Symbol,
    Fn,
    Lambda,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Lambda {
    pub params: Rc<Expr>,
    pub body: Rc<Expr>,
}

macro_rules! to {
    ($name:ident => $pat:ident($inner:ident) => $t:ty) => {
        pub fn $name(&self) -> LangResult<&$t> {
            match self {
                Expr::$pat($inner) => Ok($inner),
                _ => Err(LangError::TypeMismatched),
            }
        }
    };
}

impl Expr {
    to!(inner_num => Num(n) => f64);
    to!(inner_string => String(s) => Rc<str>);
    to!(inner_bool => Bool(b) => bool);
    to!(inner_q_expr => QExpr(q_expr) => Box<Expr>);
    to!(inner_s_expr => SExpr(s_expr) => Vec<Expr>);
    to!(inner_symbol => Symbol(s) => String);
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Expr::Nil => "nil".to_string(),
            Expr::Symbol(s) => s.to_string(),
            Expr::Num(n) => n.to_string(),
            Expr::String(s) => format!("\"{}\"", s.clone()),
            Expr::Bool(b) => String::from(if *b { "#t" } else { "#f" }),
            Expr::SExpr(s_expr) => {
                let xs: Vec<String> = s_expr.iter().map(|s| s.to_string()).collect();
                format!("({})", xs.join(" "))
            }
            Expr::QExpr(q_expr) => {
                let q = q_expr.to_string();
                format!("'{q}")
            }
            Expr::Fn(f) => format!("Function: {:?}", f), // _ => unimplemented!(),
            Expr::Lambda(lambda) => {
                let params = lambda.params.clone();
                let body = lambda.body.clone();
                format!("Lambda: params: {} body: {}", params, body)
            }
        };
        write!(f, "{}", str)
    }
}

pub fn from(parsed_exprs: Pair<Rule>) -> LangResult<Vec<Expr>> {
    let mut ast = vec![];

    for expr in parsed_exprs.into_inner() {
        for inner_expr in expr.into_inner() {
            let new_expr = match inner_expr.as_rule() {
                Rule::nil => Expr::Nil,
                Rule::num => {
                    let num = inner_expr
                        .as_str()
                        .parse()
                        .map_err(|e| LangError::ParseFailed(format!("{e}")))?;
                    Expr::Num(num)
                }
                Rule::string => Expr::String(inner_expr.into_inner().as_str().into()),
                Rule::bool => Expr::Bool(inner_expr.as_str() == "#t"),
                Rule::symbol => Expr::Symbol(inner_expr.as_str().to_string()),
                Rule::s_expr => {
                    // let input = add_bracket(inner_expr.as_str());
                    let input = inner_expr.as_str();
                    let s_expr =
                        // LangParser::parse(Rule::s_expr, trim_bracket_outer(inner_expr.as_str()))
                        LangParser::parse(Rule::s_expr, input)
                            .map_err(|e| LangError::ParseFailed(format!("{e}")))?
                            .next()
                            .unwrap();
                    let s_expr = from(s_expr)?;
                    Expr::SExpr(s_expr)
                }
                Rule::q_expr => Expr::QExpr(Box::new(from(inner_expr)?[0].clone())),
                _ => unimplemented!(),
            };

            ast.push(new_expr);
        }
    }

    Ok(ast)
}
