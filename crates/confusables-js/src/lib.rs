#![allow(clippy::unnecessary_wraps)]

use neon::prelude::*;

fn hello(mut cx: FunctionContext<'_>) -> JsResult<'_, JsString> {
  Ok(cx.string("hello node"))
}

fn hello_name(mut cx: FunctionContext<'_>) -> JsResult<'_, JsString> {
  let name = cx.argument::<JsString>(0)?.value(&mut cx);
  Ok(cx.string(format!("hello {name}")))
}

#[neon::main]
fn main(mut cx: ModuleContext<'_>) -> NeonResult<()> {
  cx.export_function("hello", hello)?;
  cx.export_function("hello_name", hello_name)?;
  Ok(())
}
