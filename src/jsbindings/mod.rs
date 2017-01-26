use visitor;
use diagnostics;
use ::{Rdp, StringInput};
use ::get_diagnostics_for_tsql as lib_get_diagnostics;
use visitors;

use neon::js::{JsString, JsNumber, JsObject, Key};
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
    pub fn to_jsobject(self, call: Call) -> Handle<JsObject> {
        let scope = call.scope;
        let obj = JsObject::new(scope);

        let pos_line = JsNumber::new(call.scope, self.pos.line as f64);
        let pos_col = JsNumber::new(call.scope, self.pos.col as f64);
        let code = JsString::new(call.scope, &self.code);
        let message = JsString::new(call.scope, &self.message);
        obj.set("pos_line", pos_line);
        obj.set("pos_col", pos_col);
        obj.set("code", code);
        obj.set("message", message);
        obj
    }
}