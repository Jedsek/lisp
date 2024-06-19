use std::fmt::Display;

use crate::{LangError, LangParser, LangResult, Rule};
use pest::iterators::Pair;
use pest::Parser;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Expr {
    Num(f64),
    String(String),
    Bool(bool),
    QExpr(Vec<Expr>),
    SExpr(Vec<Expr>),
    Symbol(String),
    Fn(fn(&[Expr]) -> LangResult<Expr>),
    Nil,
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Expr::Symbol(s) => s.to_string(),
            Expr::Num(n) => n.to_string(),
            Expr::Bool(b) => String::from(if *b { "#t" } else { "#f" }),
            Expr::SExpr(s_expr) => {
                let xs = s_expr.iter().map(|s| s.to_string()).collect::<Vec<_>>();
                format!("({})", xs.join(", "))
            }
            _ => unimplemented!(),
        };
        write!(f, "{}", str)
    }
}

// macro_rules! mk_expr {
//     (with: $matched:expr ; {
//         $( $pat:ident $(| $pat_others:ident)*  $(=> $expr:expr)? )+
//     }) => {
//         match $matched {
//             $(Rule::$pat $(| Rule::$pat_others)* => Expr::$pat $(($expr))? ),*,
//             Rule::EOI => break,
//             _ => continue,
//         }
//     };
// }

fn trim_bracket_outer(s: &str) -> &str {
    fn trim_recursive(s: &str, right_pos: usize) -> &str {
        if s.starts_with("((") && s.ends_with("))") {
            trim_recursive(&s[1..right_pos], right_pos - 1)
        } else {
            s
        }
    }
    trim_recursive(s, s.len() - 1)
}

pub fn from(parsed_exprs: Pair<Rule>) -> LangResult<Vec<Expr>> {
    let mut ast = vec![];

    for expr in parsed_exprs.into_inner() {
        for inner_expr in expr.into_inner() {
            /*
            let new_expr = mk_expr!(with: inner_expr.as_rule() ; {
                nil
                num => {
                    inner_expr.as_str().parse()
                        .map_err(|e| LangError::ParseFailed(format!("{e}")))?
                }
                string => inner_expr.as_str().to_string()
                bool => inner_expr.as_str() == "#t"
                q_expr => from(inner_expr)?
                s_expr => {
                    let s_expr =
                        LangParser::parse(Rule::s_expr, trim_bracket_outer(inner_expr.as_str()))
                            .map_err(|e| LangError::ParseFailed(format!("{e}")))?
                            .next().unwrap();
                    from(s_expr)?
                }
                symbol => inner_expr.as_str().to_string()
            });
            */
            let new_expr = match inner_expr.as_rule() {
                Rule::nil => Expr::Nil,
                Rule::num => {
                    let num = inner_expr
                        .as_str()
                        .parse()
                        .map_err(|e| LangError::ParseFailed(format!("{e}")))?;
                    Expr::Num(num)
                }
                Rule::string => Expr::String(inner_expr.as_str().to_string()),
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

pub fn eval_expr(expr: &Expr) -> LangResult<f64> {
    let result = match expr {
        Expr::Num(num) => *num,
        Expr::SExpr(exprs) => eval_sexpr(exprs)?,
        _ => return Err(LangError::FuckYou), // Fuck you!
    };
    Ok(result)
}

fn eval_sexpr(exprs: &[Expr]) -> LangResult<f64> {
    let len = exprs.len();

    // if only one number in S-expr
    if let Expr::Num(num) = &exprs[0] {
        return if len == 1 {
            Ok(*num)
        } else {
            Err(LangError::FuckYou) // Fuck you!
        };
    }

    let Expr::Symbol(symbol) = &exprs[0] else {
        return Err(LangError::FuckYou); // Fuck you!
    };

    // negative symbol
    let args = exprs.iter().skip(1).map(eval_expr).collect::<Vec<_>>();
    if len == 2 && symbol.as_str() == "-" {
        if let Ok(e) = args[0].as_ref() {
            return Ok(-e);
        }
    }

    // add/sub/mul/div...and more
    let operate: fn(f64, f64) -> LangResult<f64> = match symbol.as_str() {
        "+" => |acc, e| Ok(acc + e),
        "-" => |acc, e| Ok(acc - e),
        "*" => |acc, e| Ok(acc * e),
        "/" => |acc, e| {
            if e == 0.0 {
                Err(LangError::DivideByZero)
            } else {
                Ok(acc / e)
            }
        },
        "%" => |acc, e| Ok(acc % e),
        "^" => |acc: f64, e| Ok(acc.powf(e)),
        "max" => |acc, e| Ok(acc.max(e)),
        "min" => |acc, e| Ok(acc.min(e)),
        symbol_name => return Err(LangError::InvalidSymbol(symbol_name.into())),
    };

    // LL with error handling
    // `L`: Left to rgiht
    // `L`: Left reduce and combination
    args.into_iter()
        .reduce(|acc, expr| match (acc, expr) {
            (Ok(x), Ok(y)) => operate(x, y),
            (Err(err), _) | (_, Err(err)) => Err(err),
        })
        .unwrap()
}
