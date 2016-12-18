// tsqlust -- GPLv3 T-SQL static analysis framework
// Copyright (C) 2016 Taryn Hill

#![allow(unused_must_use)]

extern crate pest;
use pest::{Parser, StringInput};

extern crate tsqlust;
use tsqlust::{Rdp, Rule};

use std::io::{self, BufRead, Write, StdoutLock};

fn main() {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    stdout.write(b"Enter q at any time to quit.\n");
    stdout.write(b"Enter ? at any time for help.\n");
    stdout.write(b"Note: Expressions cannot be parsed currently. See https://github.com/Phrohdoh/tsqlust/issues/13\n");
    stdout.write(b"\n");

    loop {
        stdout.write(b">> ");
        stdout.flush().expect("Failed to flush stdout");

        let mut line = String::new();
        stdin.read_line(&mut line).expect("Failed to read from stdin");
        line = line.trim().to_string();

        if line.is_empty() {
            continue;
        }

        let line_str = line.as_str();
        match line_str {
            "help" | "?" | "/h" | "/?" => {
                stdout.write(b"Enter q at any time to quit.\n");
            }
            "q" | "quit" | "exit" => {
                stdout.write(b"Goodbye!\n");
                break;
            }
            _ => {
                let mut parser = Rdp::new(StringInput::new(line_str));
                if !try_print_ast(&mut parser, &mut stdout) {
                    stdout.write("tsqlust did not expect this input, please report this.\n"
                        .as_bytes());
                    assert!(!parser.top_level_repl());
                    stdout.write(format!("Queue:\n{:#?}\n", parser.queue()).as_bytes());
                    stdout.write(format!("Expected:\n{:#?}\n", parser.expected()).as_bytes());
                    break;
                }
            }
        }
    }
}

fn try_print_ast(parser: &mut Rdp<StringInput>, stdout: &mut StdoutLock) -> bool {
    if parser.top_level_repl() {
        let first = parser.queue().get(0).unwrap();
        match first.rule {
            Rule::stmt_select => {
                stdout.write(format!("{:#?}\n", parser.parse_stmt_select()).as_bytes());
            }
            Rule::stmt_top_legacy |
            Rule::stmt_top => {
                stdout.write(format!("{:#?}\n", parser.parse_stmt_top()).as_bytes());
            }
            r @ _ => {
                stdout.write(format!("{:#?}\n", r).as_bytes());
            }
        }
        true
    } else {
        false
    }
}
