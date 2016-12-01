// tsqlust -- GPLv3 T-SQL static analysis framework
// Copyright (C) 2016 Taryn Hill

extern crate tsqlust;

use std::env;
use std::fs::File;
use std::io::Read;
use tsqlust::{ast, visitor, diagnostics, get_diagnostics_for_tsql};

use ast::{SelectStatement, TopStatement};
use visitor::Visitor;
use diagnostics::{Context, Diagnostic, DiagnosticType};

struct ExampleVisitor { }

impl Visitor for ExampleVisitor {
    fn visit_select_statement(&mut self, _: &mut Context, _: &SelectStatement) {}
    fn visit_top_statement(&mut self, ctx: &mut Context, top_statement: &TopStatement) {
        if top_statement.is_legacy() {
            return;
        }

        let open_paren = top_statement.paren_open.as_ref().unwrap();
        let close_paren = top_statement.paren_close.as_ref().unwrap();

        if close_paren.pos.line != open_paren.pos.line {
            ctx.add_diagnostic(Diagnostic {
                diagnostic_type: DiagnosticType::Warning,
                pos: close_paren.pos,
                message: "Closing paren must be on the same line as the opening paren".into(),
            });
        }
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