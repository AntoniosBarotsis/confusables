use std::borrow::Cow;

use crate::KEYWORDS;

/// A trait that adds confusable related utilities to strings.
pub trait Confusable {
  /// Checks whether the string contains any confusable characters.
  ///
  /// # Example
  ///
  /// ```
  /// # use confusables::Confusable;
  /// assert!("𝓗℮𝐥1೦".contains_confusable());
  /// assert!(!"Hello".contains_confusable());
  /// ```
  fn contains_confusable(&self) -> bool;

  /// Checks whether the current string is confusable with another string
  ///
  /// # Example
  ///
  /// ```
  /// # use confusables::Confusable;
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
  /// # use confusables::Confusable;
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

  /// Replaces all confusable characters.
  ///
  /// # Example
  ///
  /// ```
  /// # use confusables::Confusable;
  /// assert_eq!("𝓗℮𝐥1೦".replace_confusable(), "Hello");
  ///
  /// // This will cause an allocation
  /// assert_eq!("Hello".replace_confusable(), "Hello");
  /// ```
  fn replace_confusable(&self) -> String;

  /// Replaces all confusables characters, if any.
  ///
  /// It first checks whether the input contains any confusable
  /// characters in the first place. If you are certain it does,
  /// you can call [`Confusable::replace_confusable`] directly,
  /// otherwise this might save you an allocation.
  ///
  /// # Example
  ///
  /// ```
  /// # use confusables::Confusable;
  /// assert_eq!("𝓗℮𝐥1೦".detect_replace_confusable(), "Hello");
  ///
  /// // This will *not* cause an allocation
  /// assert_eq!("Hello".detect_replace_confusable(), "Hello");
  /// ```
  fn detect_replace_confusable(&self) -> Cow<'_, str>;
}

impl<T> Confusable for T
where
  T: AsRef<str> + Clone,
{
  fn contains_confusable(&self) -> bool {
    self.as_ref().chars().any(|c| KEYWORDS.get(&c).is_some())
  }

  fn is_confusable_with(&self, other: &str) -> bool {
    let unconfused = self.detect_replace_confusable();

    unconfused == other
  }

  fn confusable_contains(&self, other: &str) -> bool {
    if other.len() > self.as_ref().len() {
      return false;
    }

    let unconfused = self.detect_replace_confusable();

    unconfused.contains(other)
  }

  fn replace_confusable(&self) -> String {
    // Create a string buffer with room for more chars than the initial string
    // since some confusables map to more than one char.
    let mut output = String::with_capacity(self.as_ref().len() * 2);

    self.as_ref().chars().for_each(|c| {
      if let Some(s) = KEYWORDS.get(&c) {
        output.push_str(s);
      } else {
        output.push(c);
      }
    });

    output
  }

  fn detect_replace_confusable(&self) -> Cow<'_, str> {
    if self.contains_confusable() {
      Cow::Owned(self.replace_confusable())
    } else {
      Cow::Borrowed(self.as_ref())
    }
  }
}
