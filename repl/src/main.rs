mod config;

use config::{CustomPrompt, LineEditorBuilder};
use lang::{ast::Expr, env::Env};
use reedline::{FileBackedHistory, Reedline, Signal};
use std::fs;

#[derive(Default)]
struct State {
    debug_enabled: bool,
    id: usize,
}

impl State {
    fn debug_toggle(&mut self) {
        self.debug_enabled = !self.debug_enabled;
        println!("debug enabled: {}\n", self.debug_enabled);
    }
}

fn main() {
    print_helps();

    let mut state = State::default();
    let mut line_editor = line_editor();
    let mut env = Env::default();
    let prompt = CustomPrompt::default();

    loop {
        let line = line_editor.read_line(&prompt);
        match line {
            Ok(Signal::CtrlD | Signal::CtrlC) => break,
            Ok(Signal::Success(buffer)) if !buffer.is_empty() => match buffer.as_str() {
                ":e" | ":q" | ":exit" | ":quit" => break,
                ":h" | ":help" => print_helps_verbose(),
                ":c" | ":clear" => {
                    line_editor.clear_screen().ok();
                }
                ":d" | ":debug" => state.debug_toggle(),
                content => {
                    let args = content.split(' ').collect::<Vec<_>>();
                    let input = if args[0] == ":l" {
                        fs::read_to_string(args[1]).unwrap()
                    } else {
                        content.to_string()
                    };
                    match lang::eval(&input, &mut env, false) {
                        Err(err) => eprintln!("{}\n", err),
                        Ok(exprs) => {
                            for expr in exprs {
                                match expr {
                                    Err(err) => eprintln!("{}\n", err),
                                    Ok(expr) => {
                                        let id = state.id;
                                        if state.debug_enabled {
                                            println!("${id} = {expr}");
                                        } else if args[0] != ":l" {
                                            println!("{expr}");
                                        } else if args[0] == ":l" {
                                            if let Expr::Symbol(ref symbol) = expr {
                                                println!("{symbol}");
                                            }
                                        }
                                        env.define(format!("${id}"), expr);
                                        state.id += 1;
                                    }
                                }
                            }
                            println!();
                        }
                    }
                }
            },
            _ => (),
        }
    }

    println!("\nAborted");
}

fn print_helps() {
    println!(
        "Lisp v0.0.1 :)
● emacs-like keybinding is enabled default
● type `:h` or `:help` for verbose help infomations
"
    )
}

fn print_helps_verbose() {
    println!(
        r#"Lisp v0.0.1
● emacs-like keybinding is enabled default
● type `:h | :help` for verbose help infomations
● type `:c | :clear` for clearing screen
● type `alt-enter` for newline insert
● use `:e | :exit | :q | :quit`, ctrl-c, or ctrl-d to exit prompt


Example code:

> 1.234
1.234

> (+ 1 2 3 4 5)
15

> (+ 1 1e-1 1e-2 1e-3 1e-4)
1.1111

> (+ 1 2 3 4 5 (- (* 2 2 (/ 9 4.5))))
7

> (def a 1)
a
> (+ a 1)
2

> (if (< 1 2 3 4 5) "Yes" "No")
"Yes"
"#
    )
}

#[allow(clippy::let_and_return)]
fn line_editor() -> Reedline {
    let history = Box::new(
        FileBackedHistory::with_file(1000, "history.txt".into())
            .expect("Error configuring history with file"),
    );

    let line_editor = Reedline::create().with_history(history);

    LineEditorBuilder(line_editor)
        .with_complete()
        .with_highlight()
        .with_hinter()
        .with_edit_mode()
        .build()
}
