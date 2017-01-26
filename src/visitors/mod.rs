use ast::{TopStatement, Node};
use visitor::Visitor;
use diagnostics::{Context, Diagnostic};

pub struct SameLineTopStmtParens { }

impl Visitor for SameLineTopStmtParens {
    fn visit_top_statement(&mut self, ctx: &mut Context, node: &Node<TopStatement>) {
        let ref top_statement = node.tnode;
        if top_statement.is_legacy() {
            return;
        }

        let open_paren = top_statement.paren_open.as_ref().unwrap();
        let close_paren = top_statement.paren_close.as_ref().unwrap();

        if close_paren.pos.line != open_paren.pos.line {
            ctx.add_diagnostic(Diagnostic {
                code: "PH0004".into(),
                pos: close_paren.pos,
                message: "Closing paren must be on the same line as the opening paren".into(),
            });
        }
    }
}