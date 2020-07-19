use rustyline::error::ReadlineError;
use rustyline::Editor;
use ansi_term::Color::RGB;

fn main() {
    // `()` can be used when no completer is required
    let mut rl = Editor::<()>::new();
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }

    let logo_color = RGB(0xff, 0x7f, 0x2a);

    println!("{}", logo_color.paint("Welcome to openshell!"));

    let logo_text = RGB(0xde, 0x87, 0x87);
    loop {
        let readline = rl.readline(&format!("{}>>", logo_text.paint("openshell")));
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                println!("Line: {}", line);
                execute(line.trim());
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }
    rl.save_history("history.txt").unwrap();
}

use std::error::Error;

pub type Result<T> = std::result::Result<T, dyn Error>;

fn execute(input: &str) -> Result<()> {
    let mut input = input.split_ascii_whitespace();
    let command = input.next().expect("command wrong");
    let args = |n| {
        let arg = input.collect();
        if arg.len() != n {
            println!("command wrong");
        } else {
            Ok(arg)
        }
    };

    match command {
        "ls" => {

        }
        "add" => {

        }
        "rm" => {

        }
        "mv" => {

        }
        "open" => {

        }
        "cd" => {

        }
        "mkdir" => {

        }
        "help" => {

        }
    }

    Ok(())
}