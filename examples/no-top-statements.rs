// tsqlust -- GPLv3 T-SQL static analysis framework
// Copyright (C) 2016 Taryn Hill

extern crate tsqlust;

use std::env;
use std::fs::File;
use std::io::Read;
use tsqlust::{ast, visitor, diagnostics, get_diagnostics_for_tsql};

use ast::{SelectStatement, TopStatement, Position};
use visitor::Visitor;
use diagnostics::{Context, Diagnostic};

struct ExampleVisitor { }

impl Visitor for ExampleVisitor {
    fn visit_top_statement(&mut self, ctx: &mut Context, _: &TopStatement) {
        ctx.add_diagnostic(Diagnostic {
            code: "EX0001".into(),
            pos: Position::from((1, 1)), // TODO: Change visitors to take in Node<T> so `pos` can be accessed
            message: "TOP statements are forbidden!".into(),
        });
    }
}

fn main() {
    match env::args().nth(1) {
        Some(file_path) => {
            let mut query_string = String::new();
            let mut file = File::open(file_path).expect("Could not find file!");
            let _ = file.read_to_string(&mut query_string);

            let mut vis = ExampleVisitor {};
            let diagnostics = get_diagnostics_for_tsql(&query_string, &mut vis)
                .expect("Failed to get diagnostics!");

            for diag in diagnostics {
                println!("(line: {}, col: {}) -> {}",
                         diag.pos.line,
                         diag.pos.col,
                         diag.message);
            }
        }
        _ => {
            println!("Please provide a path to a file containing a T-SQL query");
        }
    }
}