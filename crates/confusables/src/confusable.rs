use crate::{detect_replace, KEYWORDS};

pub trait Confusable {
  /// Checks whether the string contains any confusable characters.
  ///
  /// # Example
  /// ```
  /// use crate::confusables::Confusable;
  ///
  /// assert!("𝓗℮𝐥1೦".contains_confusable());
  /// assert!(!"Hello".contains_confusable());
  /// ```
  fn contains_confusable(&self) -> bool;

  /// Checks whether this string is confusable with the other string
  ///
  /// # Example
  ///
  /// ```
  /// use crate::confusables::Confusable;
  ///
  /// let data = [
  ///   ("ǉeto", "ljeto"),
  ///   ("pаypаl", "paypal"),
  ///   ("ѕсоре", "scope"),
  ///   ("𝓗℮𝐥1೦", "Hello"),
  ///   ("m", "rn"),
  /// ];
  ///
  ///  for (input, output) in data {
  ///    // Left "simplifies" to right
  ///    assert!(input.is_confusable_with(output));
  ///    // Right does not "simplify" to left
  ///    assert!(!output.is_confusable_with(input));
  ///  }
  /// ```
  fn is_confusable_with(&self, right: &str) -> bool;

  /// A wrapper for the [`str::contains`] method that is resilient to confusables.
  ///
  /// # Example
  ///
  /// ```
  /// use crate::confusables::Confusable;
  ///
  /// let text = "𝓗℮𝐥1೦, world!";
  /// let normal_contains = text.contains("Hello");
  /// let confusable_contains = text.confusable_contains("Hello");
  ///
  /// // Returns false
  /// assert!(!normal_contains);
  /// // Returns true!
  /// assert!(confusable_contains);
  /// ```
  fn confusable_contains(&self, other: &str) -> bool;
}

impl<T> Confusable for T
where
  T: AsRef<str>,
{
  fn contains_confusable(&self) -> bool {
    self.as_ref().chars().any(|c| KEYWORDS.get(&c).is_some())
  }

  fn is_confusable_with(&self, other: &str) -> bool {
    let unconfused = detect_replace(self.as_ref());

    unconfused == other
  }

  fn confusable_contains(&self, other: &str) -> bool {
    if other.len() > self.as_ref().len() {
      return false;
    }

    let unconfused = detect_replace(self.as_ref());

    unconfused.contains(other)
  }
}
