use crate::{
    ast::Expr,
    env::Env,
    utils::{ensure, parse_list_of_floats, parse_list_of_strings},
    LangError,
};

pub fn define_std(env: &mut Env) {
    define_string(env);
    define_list(env);
    define_compare(env);
    define_equal(env);
    define_add(env);
    define_sub(env);
    define_mul(env);
    define_div(env);
    define_io(env);
    define_other(env);
}

fn define_list(env: &mut Env) {
    #[rustfmt::skip]
    macro_rules! nth {
        ($a:expr => $n:expr) => {
            env.define($a.into(), Expr::Fn(|args| {
                let first = args.first().cloned().ok_or(LangError::InvalidArgsLen)?;
                let args = first.inner_q_expr()?;
                Ok(match *args {
                    Expr::SExpr(args) => args.get($n).cloned().ok_or(LangError::InvalidArgsLen)?,
                    e => e,
                })
            }));
        };
    }

    env.define(
        "list".into(),
        Expr::Fn(|args| {
            let args = Expr::SExpr(args.to_vec());
            Ok(Expr::QExpr(Box::new(args)))
        }),
    );

    nth!("car" => 0);
    nth!("cdr" => 1);
    nth!("cadr" => 2);
    nth!("caddr" => 3);
    nth!("cadddr" => 4);
}
fn define_other(env: &mut Env) {
    env.define(
        "begin".into(),
        Expr::Fn(|args| args.iter().last().cloned().ok_or(LangError::InvalidArgsLen)),
    );

    env.define(
        "when".into(),
        Expr::Fn(|args| {
            if args.len() < 2 {
                return Err(LangError::InvalidArgsLen);
            }
            let cond = args[0].inner_bool()?;

            let result = match cond {
                true => args.last().cloned().unwrap(),
                false => Expr::Nil,
            };
            Ok(result)
        }),
    );
}

fn define_io(env: &mut Env) {
    env.define(
        "display".into(),
        Expr::Fn(|args| {
            args.iter().for_each(|e| print!("{e}"));
            Ok(Expr::Nil)
        }),
    );
    env.define(
        "displayln".into(),
        Expr::Fn(|args| {
            args.iter().for_each(|e| print!("{e}"));
            println!();
            Ok(Expr::Nil)
        }),
    );
    env.define(
        "newline".into(),
        Expr::Fn(|args| match args.is_empty() {
            false => Err(LangError::InvalidArgsLen),
            true => {
                println!();
                Ok(Expr::Nil)
            }
        }),
    );
}

fn define_string(env: &mut Env) {
    env.define(
        "string-append".into(),
        Expr::Fn(|args| {
            let string = parse_list_of_strings(args)?
                .iter()
                .fold(String::new(), |acc, e| acc + e);
            Ok(Expr::String(string))
        }),
    );
}

fn define_add(env: &mut Env) {
    env.define(
        "+".into(),
        Expr::Fn(|args| {
            let args = parse_list_of_floats(args)?;
            let sum = args
                .into_iter()
                .reduce(|acc, e| acc + e)
                .ok_or(LangError::InvalidArgsLen)?;
            Ok(Expr::Num(sum))
        }),
    );
}

fn define_sub(env: &mut Env) {
    env.define(
        "-".into(),
        Expr::Fn(|args| {
            let args = parse_list_of_floats(args)?;
            if args.is_empty() {
                return Err(LangError::InvalidArgsLen);
            }
            let first = &args[0];
            if args.len() == 1 {
                return Ok(Expr::Num(-first));
            }
            let sum_rest = args[1..]
                .iter()
                .cloned()
                .reduce(|acc, e| acc + e)
                .ok_or(LangError::InvalidArgsLen)?;
            Ok(Expr::Num(first - sum_rest))
        }),
    );
}

fn define_mul(env: &mut Env) {
    env.define(
        "*".into(),
        Expr::Fn(|args| {
            let args = parse_list_of_floats(args)?;
            let product = args
                .into_iter()
                .reduce(|acc, e| acc * e)
                .ok_or(LangError::InvalidArgsLen)?;
            Ok(Expr::Num(product))
        }),
    );
}
fn define_div(env: &mut Env) {
    env.define(
        "/".into(),
        Expr::Fn(|args| {
            let args = parse_list_of_floats(args)?;
            let num = args
                .into_iter()
                .try_reduce(|acc, e| {
                    if e == 0.0 {
                        Err(LangError::DivideByZero)
                    } else {
                        Ok(acc / e)
                    }
                    // todo!()
                })?
                .ok_or(LangError::InvalidArgsLen)?;
            Ok(Expr::Num(num))
        }),
    );
}

fn define_compare(env: &mut Env) {
    macro_rules! ensure {
        ($x:ident $op:tt $y:ident) => {
            |args| match args.len() <= 1 {
                true => Err(LangError::InvalidArgsLen),
                false => ensure(args, |$x, $y| $x $op $y)
            }
        };
    }

    env.define(">".into(), Expr::Fn(ensure!(x > y)));
    env.define("<".into(), Expr::Fn(ensure!(x < y)));
    env.define(">=".into(), Expr::Fn(ensure!(x >= y)));
    env.define("<=".into(), Expr::Fn(ensure!(x <= y)));
}

fn define_equal(env: &mut Env) {
    // let mut env = env.borrow_mut();
    env.define("=".into(), Expr::Fn(|args| ensure(args, |x, y| x == y)));
    env.define("!=".into(), Expr::Fn(|args| ensure(args, |x, y| x != y)));
}
