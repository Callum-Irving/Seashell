mod commands;
mod parser;

use reedline::{DefaultPrompt, Reedline, Signal};

fn main() {
    let mut line_editor = Reedline::create();
    let prompt = DefaultPrompt::default();

    loop {
        let sig = line_editor.read_line(&prompt).unwrap();
        match sig {
            Signal::Success(buf) => {
                let tokens = parser::parse_line(&buf);
                if let Some(builtin) = commands::get_builtin(tokens[0]) {
                    builtin(&tokens);
                } else {
                    println!("Unknown identifier: {}", tokens[0]);
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
