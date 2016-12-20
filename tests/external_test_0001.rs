extern crate pest;
use pest::prelude::StringInput;

extern crate tsqlust;
use tsqlust::ast::{SelectStatement, Node};
use tsqlust::visitor::Visitor;
use tsqlust::diagnostics::{Context, Diagnostic};
use tsqlust::Rdp;

struct ExternalConsumer { }

impl Visitor for ExternalConsumer {
    fn visit_select_statement(&mut self, ctx: &mut Context, node: &Node<SelectStatement>) {
        let ref select_statement = node.tnode;
        if select_statement.top_statement.is_none() {
            ctx.add_diagnostic(Diagnostic {
                code: "must-have-top".into(),
                pos: node.pos,
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

    let node = parser.parse_stmt_select();
    vis.visit_select_statement(&mut ctx, &node);

    let diags = ctx.get_diagnostics();
    assert_eq!(diags.len(), 1);
}