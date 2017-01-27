use diagnostics;
use visitors;
use ::get_diagnostics_for_tsql as lib_get_diagnostics;

use neon::js::{JsString, JsNumber, JsObject, JsArray, Object};
use neon::vm::{Call, JsResult};

pub fn get_diagnostics_for_tsql<'a>(call: &'a mut Call) -> JsResult<'a, JsArray> {
    let tsql = call.arguments.require(call.scope, 0)?.check::<JsString>()?.value();

    let mut vis = visitors::SameLineTopStmtParens {};
    let diagnostics = lib_get_diagnostics(&tsql, &mut vis).expect("Failed to get diagnostics");
    convert_diagnostics_to_jsarray(diagnostics, call)
}

pub fn convert_diagnostics_to_jsarray<'a>(diagnostics: Vec<diagnostics::Diagnostic>,
                                          call: &'a mut Call)
                                          -> JsResult<'a, JsArray> {
    let len = diagnostics.len() as u32;
    let arr = JsArray::new(call.scope, len);

    for (idx, diagnostic) in diagnostics.into_iter().enumerate() {
        let item = convert_diagnostic_to_jsobject(diagnostic, call)?;
        arr.set(idx as u32, item)?;
    }

    Ok(arr)
}

pub fn convert_diagnostic_to_jsobject<'a>(diagnostic: diagnostics::Diagnostic,
                                          call: &'a mut Call)
                                          -> JsResult<'a, JsObject> {
    let obj = JsObject::new(call.scope);

    let pos_line = JsNumber::new(call.scope, diagnostic.pos.line as f64);
    let pos_col = JsNumber::new(call.scope, diagnostic.pos.col as f64);
    let code = JsString::new(call.scope, &diagnostic.code).unwrap();
    let message = JsString::new(call.scope, &diagnostic.message).unwrap();

    obj.set("pos_line", pos_line)?;
    obj.set("pos_col", pos_col)?;
    obj.set("code", code)?;
    obj.set("message", message)?;
    Ok(obj)
}