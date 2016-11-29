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
    fn visit_select_statement(&mut self, s: &ast::SelectStatement) -> Option<Diagnostic>;
    fn visit_top_statement(&mut self, s: &ast::TopStatement) -> Option<Diagnostic>;
}

#[cfg(test)]
mod tests {
    use super::{Visitor, Diagnostic, DiagnosticType};
    use super::super::*;
    use ast::{SelectStatement, TopStatement};

    struct TestVisitor { }

    impl Visitor for TestVisitor {
        fn visit_select_statement(&mut self, s: &ast::SelectStatement) -> Option<Diagnostic> {
            None
        }

        fn visit_top_statement(&mut self, s: &ast::TopStatement) -> Option<Diagnostic> {
            match s.expr.value {
                ast::Expression::Literal { lit: ast::Literal::Int(v) } => {
                    if v <= 0 {
                        return Some(Diagnostic {
                            diagnostic_type: DiagnosticType::Warning,
                            pos: s.expr.pos,
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
    fn top_0_diagnostic() {
        let mut parser = Rdp::new(StringInput::new("TOP (0)"));
        assert!(parser.stmt_top());

        let mut vis = TestVisitor {};
        let stmt_top = parser.parse_stmt_top().unwrap().value;
        let diagnostic = vis.visit_top_statement(&stmt_top).unwrap();

        assert_eq!(diagnostic.pos.to_pair(), (1, 6));
    }
}