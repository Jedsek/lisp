mod config;

use config::{CustomPrompt, LineEditorBuilder};
use reedline::{FileBackedHistory, Reedline, Signal};

#[derive(Default)]
struct State {
    debug_enabled: bool,
}

impl State {
    fn debug_toggle(&mut self) {
        self.debug_enabled = !self.debug_enabled;
        println!("debug enabled: {} now", self.debug_enabled);
    }
}

fn main() {
    print_helps();

    let mut state = State::default();
    let mut line_editor = line_editor();
    let prompt = CustomPrompt::default();

    loop {
        let line = line_editor.read_line(&prompt);
        match line {
            Ok(Signal::CtrlD | Signal::CtrlC) => exit(),
            Ok(Signal::Success(buffer)) if !buffer.is_empty() => match buffer.as_str() {
                ":e" | ":q" | ":exit" | ":quit" => exit(),
                ":h" | ":help" => print_helps_verbose(),
                ":c" | ":clear" => {
                    line_editor.clear_screen().ok();
                }
                ":d" | ":debug" => state.debug_toggle(),
                content => {
                    let result = lang::eval_expr(content, false);
                    match result {
                        Ok(result) => println!("{}\n", result),
                        Err(err) => eprintln!("{}\n", err),
                    }
                }
            },
            _ => (),
        }
    }
}

// fn clear() {
//     #[cfg(target_os = "linux")]
//     std::process::Command::new("clear").status().unwrap();

//     #[cfg(target_os = "windows")]
//     std::process::Command::new("cls").status().unwrap();
// }

fn exit() {
    println!("\nAborted");
    std::process::exit(0);
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
        "Lisp v0.0.1
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
"
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
