use std::env;
use std::io::{self, Write};

const WHERE_RESPONSE: &str = "The Social Study, 1795 Geary Blvd, San Francisco";
const WHEN_RESPONSE: &str = "Monday, May 4th at 6:00pm PST";
const HEADER_STYLE: &str = "\x1b[1;38;2;64;224;208m";
const RESET_STYLE: &str = "\x1b[0m";
const BANNER: &str = r#"
  _                     _     _      _      __  __
 | |    ___   ___ __ _| |   | |    | |    |  \/  |
 | |   / _ \ / __/ _` | |   | |    | |    | |\/| |
 | |__| (_) | (_| (_| | |   | |___ | |___ | |  | |
 |_____\___/ \___\__,_|_|   |_____||_____||_|  |_|

  _   _                                         _
 | | | | __ _ _ __   __ _  ___  _   _  ___    | |_ ___
 | |_| |/ _` | '_ \ / _` |/ _ \| | | |/ __|   | __/ __|
 |  _  | (_| | | | | (_| | (_) | |_| |\__ \_  | |_\__ \
 |_| |_|\__,_|_| |_|\__, |\___/ \__,_||___(_)  \__|___/
                     |___/
"#;

fn main() {
    print_header();

    let mut args = env::args().skip(1);

    if let Some(command) = args.next() {
        let exit_code = run_command(&command);
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
    println!("  exit    Quit the CLI");
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

                if command.eq_ignore_ascii_case("exit") || command.eq_ignore_ascii_case("quit") {
                    break;
                }

                run_command(command);
            }
            Err(error) => {
                eprintln!("Failed to read input: {error}");
                break;
            }
        }
    }
}

fn run_command(command: &str) -> i32 {
    match command {
        "/where" => {
            println!("{WHERE_RESPONSE}");
            0
        }
        "/when" => {
            println!("{WHEN_RESPONSE}");
            0
        }
        "/help" | "help" => {
            print_help();
            0
        }
        "" => 0,
        _ => {
            eprintln!("Unknown command: {command}");
            eprintln!("Try /where or /when.");
            1
        }
    }
}
