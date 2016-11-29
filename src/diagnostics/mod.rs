// tsqlust -- GPLv3 T-SQL static analysis framework
// Copyright (C) 2016 Taryn Hill

use ast::Position;

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