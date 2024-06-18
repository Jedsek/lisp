use crate::LangParser;
use crate::Rule;
use anyhow::bail;
use anyhow::Result;
use pest::iterators::Pair;
use pest::Parser;

const ERROR: &str = "\nFuck you, not supported now. P!L!E!A!S!E! only calculate for it.\n
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

OH MY GOD WHY THE FUCK DOES ANYONE REALLY READ ALL OF THIS Mister/Ms. YOU ARE FUCKING BORING AND I AM FUCKING BORING TOO FUCK IT!
";

#[allow(non_camel_case_types, dead_code)]
#[derive(Debug, Clone)]
pub enum Expr {
    num(f64),
    string(String),
    bool(bool),
    q_expr(Vec<Expr>),
    s_expr(Vec<Expr>),
    symbol(String),
    nil,
}

macro_rules! mk_expr {
    (with: $matched:expr ; {
        $( $pat:ident $(| $pat_others:ident)*  $(=> $expr:expr)? )+
    }) => {
        match $matched {
            $(Rule::$pat $(| Rule::$pat_others)* => Expr::$pat $(($expr))? ),*,
            Rule::EOI => break,
            _ => continue,
        }
    };
}

fn trim_bracket(s: &str) -> &str {
    fn trim_bracket_inner(s: &str, right_pos: usize) -> &str {
        if s.starts_with("((") && s.ends_with("))") {
            trim_bracket_inner(&s[1..right_pos], right_pos - 1)
        } else {
            s
        }
    }
    trim_bracket_inner(s, s.len() - 1)
}

pub fn from(parsed_exprs: Pair<Rule>) -> Result<Vec<Expr>> {
    let mut ast = vec![];

    for expr in parsed_exprs.into_inner() {
        for inner_expr in expr.into_inner() {
            let new_expr = mk_expr!(with: inner_expr.as_rule() ; {
                nil
                num => inner_expr.as_str().parse()?
                string => inner_expr.as_str().to_string()
                bool => inner_expr.as_str() == "#t"
                // q_expr => from(inner_expr)?
                // s_expr => from(inner_expr)?
                s_expr | q_expr => {
                    let s_expr = LangParser::parse(Rule::s_expr, trim_bracket(inner_expr.as_str()))?.next().unwrap();
                    from(s_expr)?
                }
                symbol => inner_expr.as_str().to_string()
            });
            ast.push(new_expr);
        }
    }

    Ok(ast)
}

pub fn eval_expr(expr: &Expr) -> Result<f64> {
    let result = match expr {
        Expr::num(num) => *num,
        Expr::s_expr(exprs) => eval_sexpr(exprs)?,
        _ => bail!(ERROR),
    };
    Ok(result)
}

pub fn eval_sexpr(exprs: &[Expr]) -> Result<f64> {
    let len = exprs.len();

    if let Expr::num(num) = &exprs[0] {
        if len == 1 {
            return Ok(*num);
        } else {
            bail!(ERROR);
        }
    }

    let Expr::symbol(symbol) = &exprs[0] else {
        bail!(ERROR); // Fuck you!
    };

    let args = exprs.iter().skip(1).map(eval_expr);

    let operate = match symbol.as_str() {
        "+" => |acc, e| acc + e,
        "-" => |acc, e| acc - e,
        "*" => |acc, e| acc * e,
        "/" => |acc, e| acc / e,
        _ => unimplemented!(),
    };

    args.reduce(|acc, expr| match (acc, expr) {
        (Ok(x), Ok(y)) => Ok(operate(x, y)),
        (Err(err), _) | (_, Err(err)) => Err(err),
    })
    .unwrap()
}
