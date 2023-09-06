#![deny(clippy::all)]
#![allow(clippy::needless_pass_by_value)]

use napi_derive::napi;
use confusables::Confusable;

#[napi]
pub fn contains_confusable(input: String) -> bool {
  input.contains_confusable()
}

#[napi]
pub fn is_confusable_with(input: String, other: String) -> bool {
  input.is_confusable_with(&other)
}

#[napi]
pub fn confusable_contains(input: String, other: String) -> bool {
  input.confusable_contains(&other)
}

#[napi]
pub fn replace_confusable(input: String) -> String {
  input.replace_confusable()
}

#[napi]
pub fn detect_replace_confusable(input: String) -> String {
  input.detect_replace_confusable().to_string()
}
