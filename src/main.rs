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
                let tokens = parser::parse_line(buf);
                if tokens[0] == "cd" {
                    commands::cd(&tokens[1]);
                } else if tokens[0] == "ls" {
                    commands::pwd();
                } else if tokens[0] == "exit" {
                    break;
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
