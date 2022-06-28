use neon::prelude::*;

use std::alloc::System;

#[global_allocator]
static A: System = System;

fn copy_reference(mut cx: FunctionContext) -> JsResult<JsString> {
    let text: Handle<JsString> = cx.argument(0)?;
    let line: Handle<JsNumber> = cx.argument(1)?;
    let caret_position: Handle<JsNumber> = cx.argument(2)?;

    let text = text.value(&mut cx);
    let line = line.value(&mut cx);
    let caret_position = caret_position.value(&mut cx);

    let constant = ast_parser_core::copy_reference(&text, line as usize, caret_position as usize);

    if let Some(constant) = constant {
        Ok(cx.string(constant))
    } else {
        Ok(cx.string("null"))
    }
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("copyReference", copy_reference)?;
    Ok(())
}
