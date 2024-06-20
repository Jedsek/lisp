use crate::{
    ast::Expr,
    env::{Env, EnvExt},
    utils::{ensure, parse_list_of_floats, parse_list_of_strings},
    LangError,
};

pub fn define_operaters(env: Env) {
    define_concat_string(env.clone());
    define_compare(env.clone());
    define_equal(env.clone());
    define_add(env.clone());
    define_sub(env.clone());
    define_mul(env.clone());
    define_div(env.clone());
    define_io(env.clone());
}

fn define_io(env: Env) {
    env.define(
        "println".into(),
        Expr::Fn(|args| {
            args.iter().for_each(|e| println!("{e}"));
            Ok(Expr::Nil)
        }),
    );
}

fn define_concat_string(env: Env) {
    env.define(
        "++".into(),
        Expr::Fn(|args| {
            let string = parse_list_of_strings(args)?
                .iter()
                .fold(String::new(), |acc, e| acc + e);
            Ok(Expr::String(string))
        }),
    );
}

fn define_add(env: Env) {
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

fn define_sub(env: Env) {
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

fn define_mul(env: Env) {
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
fn define_div(env: Env) {
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

fn define_compare(env: Env) {
    macro_rules! ensure {
        ($x:ident $op:tt $y:ident) => {
            |args| match args.len() <= 1 {
                true => Err(LangError::InvalidArgsLen),
                false => ensure(args, |$x, $y| $x $op $y)
            }
        };
    }

    let mut env = env.borrow_mut();
    env.define(">".into(), Expr::Fn(ensure!(x > y)));
    env.define("<".into(), Expr::Fn(ensure!(x < y)));
    env.define(">=".into(), Expr::Fn(ensure!(x >= y)));
    env.define("<=".into(), Expr::Fn(ensure!(x <= y)));
}

fn define_equal(env: Env) {
    let mut env = env.borrow_mut();
    env.define("=".into(), Expr::Fn(|args| ensure(args, |x, y| x == y)));
    env.define("!=".into(), Expr::Fn(|args| ensure(args, |x, y| x != y)));
}
