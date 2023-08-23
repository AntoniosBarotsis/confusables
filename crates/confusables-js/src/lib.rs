#![allow(clippy::unnecessary_wraps)]

use confusables::Confusable;
use neon::prelude::*;

fn contains_confusable(mut cx: FunctionContext<'_>) -> JsResult<'_, JsBoolean> {
  let text = cx.argument::<JsString>(0)?.value(&mut cx);

  Ok(cx.boolean(text.contains_confusable()))
}

fn is_confusable_with(mut cx: FunctionContext<'_>) -> JsResult<'_, JsBoolean> {
  let left = cx.argument::<JsString>(0)?.value(&mut cx);
  let right = cx.argument::<JsString>(1)?.value(&mut cx);

  Ok(cx.boolean(left.is_confusable_with(&right)))
}

fn confusable_contains(mut cx: FunctionContext<'_>) -> JsResult<'_, JsBoolean> {
  let left = cx.argument::<JsString>(0)?.value(&mut cx);
  let right = cx.argument::<JsString>(1)?.value(&mut cx);

  Ok(cx.boolean(left.confusable_contains(&right)))
}

fn replace_confusable(mut cx: FunctionContext<'_>) -> JsResult<'_, JsString> {
  let text = cx.argument::<JsString>(0)?.value(&mut cx);

  Ok(cx.string(text.replace_confusable()))
}

fn detect_replace_confusable(mut cx: FunctionContext<'_>) -> JsResult<'_, JsString> {
  let text = cx.argument::<JsString>(0)?.value(&mut cx);

  Ok(cx.string(text.detect_replace_confusable()))
}

#[neon::main]
fn main(mut cx: ModuleContext<'_>) -> NeonResult<()> {
  cx.export_function("contains_confusable", contains_confusable)?;
  cx.export_function("is_confusable_with", is_confusable_with)?;
  cx.export_function("confusable_contains", confusable_contains)?;
  cx.export_function("replace_confusable", replace_confusable)?;
  cx.export_function("detect_replace_confusable", detect_replace_confusable)?;
  Ok(())
}
