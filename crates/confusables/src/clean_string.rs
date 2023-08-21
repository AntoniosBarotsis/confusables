use std::{fmt::Display, ops::Deref};

use crate::detect_replace;

#[derive(Debug)]
pub struct CleanString {
  pub(crate) inner: String,
}

impl CleanString {
  /// Returns [`true`] if the left "potentially confusable" string
  /// can be confused with the right "non confusable" string.
  ///
  /// # Note
  ///
  /// The `right` string is expected to not contain confusable characters.
  ///
  /// # Example
  ///
  /// ```
  /// # use confusables::replace;
  /// let data = [
  ///   ("ǉeto", "ljeto"),
  ///   ("pаypаl", "paypal"),
  ///   ("ѕсоре", "scope"),
  ///   ("𝓗℮𝐥1೦", "Hello"),
  ///   ("m", "rn"),
  /// ];
  ///
  ///  for (input, output) in data {
  ///    let input_replaced = replace(input);
  ///    let output_replaced = replace(output);
  ///
  ///    // Left "simplifies" to right
  ///    assert!(input_replaced.is_confusable_with(output));
  ///    // Right does not "simplify" to left
  ///    assert!(!output_replaced.is_confusable_with(input));
  ///  }
  /// ```
  pub fn is_confusable_with(&self, right: &str) -> bool {
    // The confusable mappings are 1->N so if the left already
    // has more chars, they can't possibly be equal
    if self.inner.len() > right.len() {
      return false;
    }

    let left = detect_replace(&self.inner);

    left == right
  }

  /// Checks if the [`CleanString`] contains the other string.
  ///
  /// # Note
  ///
  /// The `other` string is expected to not contain confusable characters.
  ///
  /// # Example
  ///
  /// ```
  /// # use confusables::replace;
  /// let text = replace("𝓗℮𝐥1೦, world!");
  /// assert!(text.contains("Hello"));
  /// ```
  pub fn contains(&self, other: &str) -> bool {
    if other.len() > self.inner.len() {
      return false;
    }

    let left = detect_replace(&self.inner);

    left.inner.contains(other)
  }
}

impl Display for CleanString {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    Display::fmt(&self.inner, f)
  }
}

impl PartialEq<&str> for CleanString {
  fn eq(&self, other: &&str) -> bool {
    self.inner == *other
  }
}

impl AsRef<str> for CleanString {
  fn as_ref(&self) -> &str {
    &self.inner
  }
}

impl Deref for CleanString {
  type Target = String;

  fn deref(&self) -> &Self::Target {
    &self.inner
  }
}
