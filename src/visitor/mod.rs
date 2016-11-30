// tsqlust -- GPLv3 T-SQL static analysis framework
// Copyright (C) 2016 Taryn Hill

//! src/visitor/mod.rs
//!
//! This is where the `Visitor` trait is defined.
//!
//! The purpose of this trait is so that you may write code against an AST.

use ast::{SelectStatement, TopStatement};
use diagnostics::Context;

/// The trait that allows walking an AST.
///
/// You can record diagnostic messages (warnings, errors)
/// via the [`Context`](../diagnostics/struct.Context.html)
/// struct's `add_diagnostic` function.
pub trait Visitor {
    fn visit_select_statement(&mut self, ctx: &mut Context, select_statement: &SelectStatement);
    fn visit_top_statement(&mut self, ctx: &mut Context, top_statement: &TopStatement);
}

#[cfg(test)]
mod tests {
    use super::Visitor;
    use pest::StringInput;
    use ast;
    use diagnostics::{Context, Diagnostic, DiagnosticType};
    use ::Rdp;

    struct TestVisitor { }

    impl Visitor for TestVisitor {
        fn visit_select_statement(&mut self, _: &mut Context, _: &ast::SelectStatement) {}

        fn visit_top_statement(&mut self, ctx: &mut Context, top_statement: &ast::TopStatement) {
            let ref expr_node = top_statement.expr;

            if top_statement.is_legacy {
                ctx.add_diagnostic(Diagnostic {
                    diagnostic_type: DiagnosticType::Error,
                    pos: top_statement.top_keyword_pos,
                    message: "A legacy TOP statement is simply not allowed! Add parentheses."
                        .into(),
                });
            }

            match expr_node.value {
                ast::Expression::Literal { lit: ast::Literal::Int(v) } => {
                    if v <= 0 {
                        ctx.add_diagnostic(Diagnostic {
                            diagnostic_type: DiagnosticType::Warning,
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
    fn top_0_diagnostic_warn() {
        let mut parser = Rdp::new(StringInput::new("TOP (0)"));
        assert!(parser.stmt_top());

        let mut ctx = Context::new();
        let mut vis = TestVisitor {};
        let stmt_top = parser.parse_stmt_top().unwrap().value;
        vis.visit_top_statement(&mut ctx, &stmt_top);

        let diags = ctx.get_diagnostics();
        assert_eq!(diags.len(), 1);

        let ref diag_warn = diags[0];
        assert_eq!(diag_warn.pos.to_pair(), (1, 6));
        assert_eq!(diag_warn.diagnostic_type, DiagnosticType::Warning);
    }

    #[test]
    fn top_0_diagnostic_warn_and_err() {
        let mut parser = Rdp::new(StringInput::new("TOP 0"));
        assert!(parser.stmt_top_legacy());

        let mut ctx = Context::new();
        let mut vis = TestVisitor {};
        let stmt_top = parser.parse_stmt_top().unwrap().value;
        vis.visit_top_statement(&mut ctx, &stmt_top);

        let diags = ctx.get_diagnostics();
        assert_eq!(diags.len(), 2);

        let ref diag_err = diags[0];
        assert_eq!(diag_err.pos.to_pair(), (1, 1));
        assert_eq!(diag_err.diagnostic_type, DiagnosticType::Error);

        let ref diag_warn = diags[1];
        assert_eq!(diag_warn.pos.to_pair(), (1, 5));
        assert_eq!(diag_warn.diagnostic_type, DiagnosticType::Warning);
    }
}
