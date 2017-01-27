use diagnostics;
use visitors;
use ::get_diagnostics_for_tsql as lib_get_diagnostics;

use neon::js::{JsString, JsNumber, JsObject, JsArray, Object};
use neon::vm::Call;
use neon::mem::Handle;

pub fn get_diagnostics_for_tsql<'a>(call: &'a mut Call) -> Handle<'a, JsArray> {
    let tsql = call.arguments
        .require(call.scope, 0)
        .unwrap_or_else(|_| panic!())
        .check::<JsString>()
        .unwrap_or_else(|_| panic!())
        .value();

    let mut vis = visitors::SameLineTopStmtParens {};
    let diagnostics = lib_get_diagnostics(&tsql, &mut vis).expect("Failed to get diagnostics");
    vec_diags_to_jsarray(diagnostics, call)
}

fn vec_diags_to_jsarray<'a>(diagnostics: Vec<diagnostics::Diagnostic>,
                            call: &'a mut Call)
                            -> Handle<'a, JsArray> {
    let len = diagnostics.len() as u32;
    let arr = JsArray::new(call.scope, len);

    for (idx, diag) in diagnostics.iter().enumerate() {
        let obj = JsObject::new(call.scope);

        let pos_line = JsNumber::new(call.scope, diag.pos.line as f64);
        let pos_col = JsNumber::new(call.scope, diag.pos.col as f64);
        let code = JsString::new(call.scope, &diag.code).unwrap();
        let message = JsString::new(call.scope, &diag.message).unwrap();

        let _ = obj.set("pos_line", pos_line);
        let _ = obj.set("pos_col", pos_col);
        let _ = obj.set("code", code);
        let _ = obj.set("message", message);
        let _ = arr.set(idx as u32, obj);
    }

    arr
}