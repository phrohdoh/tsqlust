use ast;

#[derive(Debug, PartialEq)]
pub struct Diagnostic {
    pub diagnostic_type: DiagnosticType,
    pub pos: ast::Position,
    pub message: String,
}

#[derive(Debug, PartialEq)]
pub enum DiagnosticType {
    Warning,
    Error,
}

pub trait Visitor {
    fn visit_select_statement(&mut self,
                              select_statement: &ast::SelectStatement)
                              -> Option<Diagnostic>;
    fn visit_top_statement(&mut self, top_statement: &ast::TopStatement) -> Option<Diagnostic>;
}

#[cfg(test)]
mod tests {
    use super::{Visitor, Diagnostic, DiagnosticType};
    use pest::StringInput;
    use ast;
    use ::Rdp;

    struct TestVisitor { }

    impl Visitor for TestVisitor {
        fn visit_select_statement(&mut self, _: &ast::SelectStatement) -> Option<Diagnostic> {
            None
        }

        fn visit_top_statement(&mut self, top_statement: &ast::TopStatement) -> Option<Diagnostic> {
            let ref expr_node = top_statement.expr;

            if top_statement.is_legacy {
                return Some(Diagnostic {
                    diagnostic_type: DiagnosticType::Error,
                    pos: top_statement.expr.pos, // TODO: This needs to be the pos of the kw_top
                    message: "A legacy TOP statement is simply not allowed! Add parentheses."
                        .into(),
                });
            }

            match expr_node.value {
                ast::Expression::Literal { lit: ast::Literal::Int(v) } => {
                    if v <= 0 {
                        return Some(Diagnostic {
                            diagnostic_type: DiagnosticType::Warning,
                            pos: expr_node.pos,
                            message: "A value <= 0 will yield an empty recordset".into(),
                        });
                    }
                }
                _ => {}
            }

            None
        }
    }

    #[test]
    fn top_0_diagnostic_warning() {
        let mut parser = Rdp::new(StringInput::new("TOP (0)"));
        assert!(parser.stmt_top());

        let mut vis = TestVisitor {};
        let stmt_top = parser.parse_stmt_top().unwrap().value;
        let diagnostic = vis.visit_top_statement(&stmt_top).unwrap();

        assert_eq!(diagnostic.pos.to_pair(), (1, 6));
        assert_eq!(diagnostic.diagnostic_type, DiagnosticType::Warning);
    }

    #[test]
    fn top_0_diagnostic_error() {
        let mut parser = Rdp::new(StringInput::new("TOP 0"));
        assert!(parser.stmt_top_legacy());

        let mut vis = TestVisitor {};
        let stmt_top = parser.parse_stmt_top().unwrap().value;
        let diagnostic = vis.visit_top_statement(&stmt_top).unwrap();

        assert_eq!(diagnostic.pos.to_pair(), (1, 5));
        assert_eq!(diagnostic.diagnostic_type, DiagnosticType::Error);
    }
}
