extern crate chrono;
use chrono::{Local, Weekday, Datelike};

extern crate pest;
use pest::prelude::StringInput;

extern crate tsqlust;
use tsqlust::ast::{SelectStatement, TopStatement, Position};
use tsqlust::visitor::Visitor;
use tsqlust::diagnostics::{Context, Diagnostic};
use tsqlust::Rdp;

struct ExternalConsumer { }

impl Visitor for ExternalConsumer {
    fn visit_select_statement(&mut self, ctx: &mut Context, select_statement: &SelectStatement) {
        if select_statement.top_statement.is_some() {
            return;
        }

        let local_now = Local::now();

        if local_now.weekday() == Weekday::Fri {
            ctx.add_diagnostic(Diagnostic {
                code: "FD0001".into(),
                pos: Position::from((1, 1)), // TODO: Change visitors to take in Node<T> so `pos` can be accessed
                message: "It is Friday. Relax with an ice-cold TOP.".into(),
            });
        }
    }

    fn visit_top_statement(&mut self, _: &mut Context, _: &TopStatement) {}
}

#[test]
fn external_test_0001() {
    let mut parser = Rdp::new(StringInput::new("SELECT * FROM MyTable"));
    assert!(parser.stmt_select());

    let mut ctx = Context::new();
    let mut vis = ExternalConsumer {};

    let stmt_select = parser.parse_stmt_select().value;
    vis.visit_select_statement(&mut ctx, &stmt_select);

    let diags = ctx.get_diagnostics();
    assert_eq!(diags.len(), 1);
}