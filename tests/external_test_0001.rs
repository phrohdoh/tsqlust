extern crate pest;
use pest::prelude::StringInput;

extern crate tsqlust;
use tsqlust::ast::{SelectStatement, Position};
use tsqlust::visitor::Visitor;
use tsqlust::diagnostics::{Context, Diagnostic};
use tsqlust::Rdp;

struct ExternalConsumer { }

impl Visitor for ExternalConsumer {
    fn visit_select_statement(&mut self, ctx: &mut Context, select_statement: &SelectStatement) {
        if select_statement.top_statement.is_none() {
            ctx.add_diagnostic(Diagnostic {
                code: "must-have-top".into(),
                pos: Position::from((1, 1)), // TODO: Change visitors to take in Node<T> so `pos` can be accessed
                message: "TOP statements are required, you don't want to pull down everything!".into(),
            });
        }
    }
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