use std::fmt::Display;
use std::rc::Rc;

use crate::utils::trim_bracket_outer;
use crate::{LangError, LangParser, LangResult, Rule};
use pest::iterators::Pair;
use pest::Parser;

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Expr {
    Nil,
    Num(f64),
    String(String),
    Bool(bool),
    QExpr(Vec<Expr>),
    SExpr(Vec<Expr>),
    Symbol(String),
    Fn(fn(&[Expr]) -> LangResult<Expr>),
    Lambda(Lambda),
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Lambda {
    pub params: Rc<Expr>,
    pub body: Rc<Expr>,
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Expr::Symbol(s) => s.to_string(),
            Expr::Num(n) => n.to_string(),
            Expr::Nil => "nil".to_string(),
            Expr::String(s) => format!("\"{}\"", s.clone()),
            Expr::Bool(b) => String::from(if *b { "#t" } else { "#f" }),
            Expr::SExpr(s_expr) => {
                let xs: Vec<String> = s_expr.iter().map(|s| s.to_string()).collect();
                format!("({})", xs.join(" "))
            }
            Expr::QExpr(q_expr) => {
                let xs: Vec<String> = q_expr.iter().map(|s| s.to_string()).collect();
                format!("{{{}}}", xs.join(" "))
            }
            Expr::Fn(f) => format!("Function: {:?}", f), // _ => unimplemented!(),
            Expr::Lambda(lambda) => {
                let params = lambda.params.clone();
                let body = lambda.body.clone();
                format!("Lambda: params: {:?} body: {:?}", params, body)
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
                Rule::string => Expr::String(inner_expr.into_inner().as_str().to_string()),
                Rule::bool => Expr::Bool(inner_expr.as_str() == "#t"),
                Rule::symbol => Expr::Symbol(inner_expr.as_str().to_string()),
                Rule::s_expr => {
                    let s_expr =
                        LangParser::parse(Rule::s_expr, trim_bracket_outer(inner_expr.as_str()))
                            .map_err(|e| LangError::ParseFailed(format!("{e}")))?
                            .next()
                            .unwrap();
                    let s_expr = from(s_expr)?;
                    Expr::SExpr(s_expr)
                }
                Rule::q_expr => Expr::QExpr(from(inner_expr)?),
                _ => unimplemented!(),
            };

            ast.push(new_expr);
        }
    }

    Ok(ast)
}
