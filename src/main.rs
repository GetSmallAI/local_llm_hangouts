use std::env;
use std::io::{self, Write};

const WHERE_RESPONSE: &str = "The Social Study, 1795 Geary Blvd, San Francisco";
const WHEN_RESPONSE: &str = "Monday, May 4th at 6:00pm PST";
const HEADER_STYLE: &str = "\x1b[1;38;2;64;224;208m";
const RESET_STYLE: &str = "\x1b[0m";
const BANNER: &str = r#"
   __________________________________________________________
  /                                                        /|
 /   Local LLM Hangouts                                   / |
+--------------------------------------------------------+  |
|                                                        |  |
|   /where   Find the next meetup location               |  |
|   /when    See the next meetup time                    |  |
|   /exit    Quit the CLI                                |  /
+--------------------------------------------------------+ /
 \_______________________________________________________\/
"#;

enum CommandResult {
    Continue(i32),
    Exit(i32),
}

fn main() {
    print_header();

    let mut args = env::args().skip(1);

    if let Some(command) = args.next() {
        let exit_code = match run_command(&command) {
            CommandResult::Continue(code) | CommandResult::Exit(code) => code,
        };
        std::process::exit(exit_code);
    }

    print_help();
    repl();
}

fn print_header() {
    println!("{HEADER_STYLE}{BANNER}{RESET_STYLE}");
}

fn print_help() {
    println!("Commands:");
    println!("  /where  Show the venue");
    println!("  /when   Show the date and time");
    println!("  /exit   Quit the CLI");
    println!();
}

fn repl() {
    let stdin = io::stdin();
    let mut input = String::new();

    loop {
        input.clear();
        print!("> ");
        io::stdout().flush().expect("failed to flush stdout");

        match stdin.read_line(&mut input) {
            Ok(0) => {
                println!();
                break;
            }
            Ok(_) => {
                let command = input.trim();

                match run_command(command) {
                    CommandResult::Continue(_) => {}
                    CommandResult::Exit(_) => break,
                }
            }
            Err(error) => {
                eprintln!("Failed to read input: {error}");
                break;
            }
        }
    }
}

fn run_command(command: &str) -> CommandResult {
    match command {
        "/where" => {
            println!("{WHERE_RESPONSE}");
            CommandResult::Continue(0)
        }
        "/when" => {
            println!("{WHEN_RESPONSE}");
            CommandResult::Continue(0)
        }
        "/help" | "help" => {
            print_help();
            CommandResult::Continue(0)
        }
        "/exit" | "exit" | "quit" => CommandResult::Exit(0),
        "" => CommandResult::Continue(0),
        _ => {
            eprintln!("Unknown command: {command}");
            eprintln!("Try /where, /when, or /exit.");
            CommandResult::Continue(1)
        }
    }
}
