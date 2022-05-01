mod commands;
mod parser;

use pathsearch::find_executable_in_path;
use reedline::{DefaultPrompt, Reedline, Signal};
use std::collections::HashMap;
use std::process::Command;

#[derive(Default)]
struct Context {
    pub last_exit: i32,
    pub aliases: HashMap<String, String>,
}

fn main() {
    let mut line_editor = Reedline::create();
    let prompt = DefaultPrompt::default();
    let mut ctx = Context::default();

    ctx.aliases.insert("ls".to_owned(), "exa".to_owned());

    loop {
        let sig = line_editor.read_line(&prompt).unwrap();
        match sig {
            Signal::Success(buf) => {
                let mut tokens = parser::parse_line(buf);
                if tokens.is_empty() {
                    continue;
                }

                // Replace variables with their values
                tokens = tokens
                    .into_iter()
                    .map(|token| {
                        if token.starts_with("$") {
                            // Replace
                            let value = std::env::var(&token[1..]).unwrap();
                            value
                        } else {
                            token
                        }
                    })
                    .collect();

                // Replace the first arg if aliased
                if let Some(alias) = ctx.aliases.get(&tokens[0]) {
                    tokens[0] = alias.clone();
                }

                // Execute command
                if let Some(builtin) = commands::get_builtin(&tokens[0]) {
                    ctx.last_exit = builtin(&tokens, &mut ctx);
                } else if let Some(path) = find_executable_in_path(&tokens[0]) {
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
