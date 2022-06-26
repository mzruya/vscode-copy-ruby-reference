use neon::prelude::*;
use std::path::Path;

fn copy_reference(mut cx: FunctionContext) -> JsResult<JsString> {
    let file_path: Handle<JsString> = cx.argument(0)?;
    let line: Handle<JsNumber> = cx.argument(1)?;
    let character: Handle<JsNumber> = cx.argument(2)?;

    let file_path = file_path.value(&mut cx);
    let line = line.value(&mut cx);
    let character = character.value(&mut cx);

    let constant = ast_parser_core::copy_reference(Path::new(&file_path), line as usize, character as usize);

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
