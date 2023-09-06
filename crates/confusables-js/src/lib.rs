#![deny(clippy::all)]
#![allow(clippy::needless_pass_by_value)]

use confusables::Confusable;
use napi_derive::napi;

/// Checks whether the string contains any confusable characters.
#[napi]
pub fn contains_confusable(input: String) -> bool {
  input.contains_confusable()
}

/// Checks whether the current string is confusable with another string.
#[napi]
pub fn is_confusable_with(input: String, other: String) -> bool {
  input.is_confusable_with(&other)
}

/// A `str.includes` method that is resilient to confusables.
#[napi]
pub fn confusable_contains(input: String, other: String) -> bool {
  input.confusable_contains(&other)
}

/// Replaces all confusable characters.
#[napi]
pub fn replace_confusable(input: String) -> String {
  input.replace_confusable()
}

/// Replaces all confusables characters, if any.
///
/// It first checks whether the input contains any confusable
/// characters in the first place. If you are certain it does,
/// you can call `replace_confusable` directly,
/// otherwise this might save you an allocation.
#[napi]
pub fn detect_replace_confusable(input: String) -> String {
  input.detect_replace_confusable().to_string()
}
