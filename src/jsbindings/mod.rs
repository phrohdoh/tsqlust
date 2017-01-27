use diagnostics;
use visitors;
use ::get_diagnostics_for_tsql as lib_get_diagnostics;

use neon::js::{JsString, JsNumber, JsObject, Object};
use neon::vm::Call;
use neon::mem::Handle;
use neon::scope::Scope;

pub fn get_diagnostics_for_tsql<'a>(call: &'a mut Call) -> Vec<Handle<'a, JsObject>> {
    let tsql = call.arguments
        .require(call.scope, 0)
        .unwrap_or_else(|_| panic!())
        .check::<JsString>()
        .unwrap_or_else(|_| panic!())
        .value();

    let mut vis = visitors::SameLineTopStmtParens {};
    lib_get_diagnostics(&tsql, &mut vis)
        .expect("Failed to get diagnostics")
        .iter()
        .map(|d| d.to_jsobject(call))
        .collect::<Vec<_>>()
}

impl diagnostics::Diagnostic {
    pub fn to_jsobject<'a>(&self, call: &'a mut Call) -> Handle<'a, JsObject> {
        let obj = JsObject::new(call.scope);

        let pos_line = JsNumber::new(call.scope, self.pos.line as f64);
        let pos_col = JsNumber::new(call.scope, self.pos.col as f64);
        let code = JsString::new(call.scope, &self.code).unwrap();
        let message = JsString::new(call.scope, &self.message).unwrap();

        let _ = obj.set("pos_line", pos_line);
        let _ = obj.set("pos_col", pos_col);
        let _ = obj.set("code", code);
        let _ = obj.set("message", message);
        obj
    }
}