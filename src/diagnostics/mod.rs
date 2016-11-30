// tsqlust -- GPLv3 T-SQL static analysis framework
// Copyright (C) 2016 Taryn Hill

use ast::Position;

/// Contains diagnostics recorded while walking an AST
/// with a struct that implements [`Visitor`](../visitor/trait.Visitor.html).
pub struct Context {
    diagnostics: Vec<Diagnostic>,
}

impl Context {
    pub fn new() -> Context {
        Context { diagnostics: vec![] }
    }

    pub fn get_diagnostics(self) -> Vec<Diagnostic> {
        self.diagnostics
    }

    pub fn add_diagnostic(&mut self, diag: Diagnostic) {
        self.diagnostics.push(diag);
    }
}

#[derive(Debug, PartialEq)]
/// Information recorded while walking an AST.
///
/// This can be anything from a warning "You should use `TOP (10)` instead of `TOP 10`"
/// to an error "`SELECT *`s are prohibited!".
///
/// These messages are created by the consumer code *not* by tsqlust.
pub struct Diagnostic {
    pub diagnostic_type: DiagnosticType,
    pub pos: Position,
    pub message: String,
}

#[derive(Debug, PartialEq)]
pub enum DiagnosticType {
    Warning,
    Error,
}