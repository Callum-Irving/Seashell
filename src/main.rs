mod commands;
mod parser;

use pathsearch::find_executable_in_path;
use reedline::{DefaultPrompt, Reedline, Signal};
use std::process::Command;

#[derive(Default)]
struct Context {
    pub last_exit: i32,
}

fn main() {
    let mut line_editor = Reedline::create();
    let prompt = DefaultPrompt::default();
    let mut ctx = Context::default();

    loop {
        let sig = line_editor.read_line(&prompt).unwrap();
        match sig {
            Signal::Success(buf) => {
                let tokens = parser::parse_line(&buf);
                if tokens.is_empty() {
                    continue;
                }

                // TODO: Replace variables with their literal values

                // Execute command
                if let Some(builtin) = commands::get_builtin(tokens[0]) {
                    builtin(&tokens);
                } else if let Some(path) = find_executable_in_path(tokens[0]) {
                    let status = Command::new(path)
                        .args(tokens[1..].iter())
                        .status()
                        .expect("error");
                    ctx.last_exit = status.code().unwrap();
                } else {
                    eprintln!("Seashell error: {} is not in PATH or builtin", tokens[0]);
                }
            }
            Signal::CtrlD | Signal::CtrlC => {
                println!("\nAborted");
                break;
            }
            Signal::CtrlL => {
                line_editor.clear_screen().unwrap();
            }
        }
    }
}
