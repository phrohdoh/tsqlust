extern crate pest;
use pest::StringInput;

extern crate tsqlust;
use tsqlust::Rdp;

use std::io::{self, BufRead, Write};

fn main() {
    let stdin = io::stdin();
    println!("Enter 'q' at any time to quit.");
    println!("Enter '?' at any time for help.");
    println!();

    loop {
        print!(">> ");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut line = String::new();
        stdin.lock().read_line(&mut line).expect("Failed to read from stdin");
        line = line.trim().to_string();

        if line.is_empty() {
            continue;
        }

        let line_str = line.as_str();
        match line_str {
            "help" | "?" | "/h" | "/?" => {
                println!("Type 'q' at any time to quit.");
                continue;
            }
            "q" | "quit" => {
                println!("Goodbye!");
                break;
            }
            _ => {
                let mut parser = Rdp::new(StringInput::new(line_str));
                print_ast(&mut parser);
            }
        }
    }
}

fn print_ast(parser: &mut Rdp<StringInput>) {
    if !parser.tsql() {
        println!("Invalid TSQL (TODO: explain why)");
        println!("Currently this REPL only accepts valid SELECT statements.");
        println!("Try: SELECT * FROM YourTableName");
        return;
    }

    println!("{:#?}", parser.parse_stmt_select());
}