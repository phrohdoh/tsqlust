extern crate tsqlust;
use tsqlust::Rdp;

use std::io::{self, BufRead, Write};

extern crate pest;
use pest::StringInput;

fn main() {
    let stdin = io::stdin();
    println!("Enter 'q' at any time to quit.");
    println!("Enter '?' at any time for help.");

    loop {
        println!();

        print!(">> ");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut line = String::new();
        stdin.lock().read_line(&mut line).expect("Failed to read from stdin");
        line = line.trim().to_string();

        if line == "q" {
            println!("Goodbye!");
            break;
        } else if line == "?" {
            println!("Type 'q' at any time to quit.");
            continue;
        }

        let mut parser = Rdp::new(StringInput::new(&line));
        if !parser.tsql() {
            println!("Invalid TSQL (TODO: explain why)");
            println!("Currently this REPL only accepts valid SELECT statements.");
            println!("Try: SELECT * FROM YourTableName");
            continue;
        }

        println!("{:#?}", parser.parse_stmt_select());
    }
}