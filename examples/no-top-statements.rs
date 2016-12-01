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
    fn visit_top_statement(&mut self, _: &mut Context, _: &TopStatement) {}
    fn visit_select_statement(&mut self, ctx: &mut Context, select_statement: &SelectStatement) {
        if let Some(ref top_statement) = select_statement.top_statement {
            ctx.add_diagnostic(Diagnostic {
                diagnostic_type: DiagnosticType::Error,
                pos: top_statement.value.top_keyword_pos,
                message: "No TOP statements!".into(),
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
            let diagnostics = get_diagnostics_for_tsql(&query_string, &mut vis);
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