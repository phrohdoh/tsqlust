// tsqlust -- GPLv3 T-SQL static analysis framework
// Copyright (C) 2016 Taryn Hill

//! src/visitor/mod.rs
//!
//! This is where the `Visitor` trait is defined.
//!
//! The purpose of this trait is so that you may write code against an AST.
//!
//! If you wanted to completely disallow `TOP` statements in your queries:
//!
//! ```rust
//! use tsqlust::get_diagnostics_for_tsql;
//! use tsqlust::ast::{SelectStatement, TopStatement, Node};
//! use tsqlust::visitor::Visitor;
//! use tsqlust::diagnostics::{Context, Diagnostic};
//!
//! struct MyVisitor { }
//!
//! impl Visitor for MyVisitor {
//!     fn visit_top_statement(&mut self,
//!                               ctx: &mut Context,
//!                               node: &Node<TopStatement>) {
//!         ctx.add_diagnostic(Diagnostic {
//!             code: "EX0001".into(),
//!             pos: node.pos,
//!             message: "TOP statements are forbidden!".into(),
//!         });
//!     }
//! }
//! ```

use ast::{SelectStatement, TopStatement, Node};
use diagnostics::Context;

/// The trait that allows walking an AST.
///
/// You can record diagnostic messages (warnings, errors)
/// via the [`Context`](../diagnostics/struct.Context.html)
/// struct's `add_diagnostic` function.
pub trait Visitor {
    fn visit_select_statement(&mut self, _ctx: &mut Context, _node: &Node<SelectStatement>) { }
    fn visit_top_statement(&mut self, _ctx: &mut Context, _node: &Node<TopStatement>) { }
}

#[cfg(test)]
mod tests {
    use super::Visitor;
    use pest::StringInput;
    use ast::{TopStatement, Node, self};
    use diagnostics::{Context, Diagnostic};
    use ::Rdp;

    struct TestVisitor { }

    impl Visitor for TestVisitor {
        fn visit_top_statement(&mut self, ctx: &mut Context, node: &Node<TopStatement>) {
            let ref stmt = node.value;
            let ref expr_node = stmt.expr;

            if stmt.is_legacy() {
                ctx.add_diagnostic(Diagnostic {
                    code: "EX0002".into(),
                    pos: node.pos,
                    message: "A legacy TOP statement is simply not allowed! Add parentheses."
                        .into(),
                });
            }

            match expr_node.value {
                ast::Expression::Literal { lit: ast::Literal::Int(v) } => {
                    if v <= 0 {
                        ctx.add_diagnostic(Diagnostic {
                            code: "EX0003".into(),
                            pos: expr_node.pos,
                            message: "A value <= 0 will yield an empty recordset".into(),
                        });
                    }
                }
                _ => {}
            }
        }
    }

    #[test]
    fn top_0_diagnostic_ex0003() {
        let mut parser = Rdp::new(StringInput::new("TOP (0)"));
        assert!(parser.stmt_top());

        let mut ctx = Context::new();
        let mut vis = TestVisitor {};
        let top_node = parser.parse_stmt_top().unwrap();
        vis.visit_top_statement(&mut ctx, &top_node);

        let diags = ctx.get_diagnostics();
        assert_eq!(diags.len(), 1);

        let ref diag_warn = diags[0];
        assert_eq!(diag_warn.pos.to_pair(), (1, 6));
        assert_eq!(diag_warn.code, "EX0003");
    }

    #[test]
    fn top_0_diagnostic_ex0002_and_ex0003() {
        let mut parser = Rdp::new(StringInput::new("TOP 0"));
        assert!(parser.stmt_top_legacy());

        let mut ctx = Context::new();
        let mut vis = TestVisitor {};
        let top_node = parser.parse_stmt_top().unwrap();
        vis.visit_top_statement(&mut ctx, &top_node);

        let diags = ctx.get_diagnostics();
        assert_eq!(diags.len(), 2);

        let ref diag_err = diags[0];
        assert_eq!(diag_err.pos.to_pair(), (1, 1));
        assert_eq!(diag_err.code, "EX0002");

        let ref diag_warn = diags[1];
        assert_eq!(diag_warn.pos.to_pair(), (1, 5));
        assert_eq!(diag_warn.code, "EX0003");
    }
}
