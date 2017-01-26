use visitor;
use diagnostics;
use ::{Rdp, StringInput};
use ::get_diagnostics_for_tsql as lib_get_diagnostics;
use visitors;

use neon;
use neon::js::{JsString, Object};
use neon::vm::Call;
use neon::mem::Handle;

pub fn get_diagnostics_for_tsql(call: Call) -> Vec<diagnostics::Diagnostic> {
    let tsql = call.arguments
        .require(call.scope, 0)
        .unwrap_or_else(|_| panic!())
        .check::<JsString>()
        .unwrap_or_else(|_| panic!())
        .value();

    let mut vis = visitors::SameLineTopStmtParens {};
    lib_get_diagnostics(&tsql, &mut vis).expect("Failed to get diagnostics")
}

impl diagnostics::Diagnostic {
    pub fn to_jsobject(self, call: Call) -> neon::js::JsObject {
        let mut obj = neon::js::JsObject::new(call.scope);
        obj.set("pos", Handle::new(vec![self.pos.line, self.pos.col]));
        obj.set("code", self.code);
        obj.set("message", self.message);
        obj
    }
}