use confusable::Confusable;
use std::borrow::Cow;

pub mod confusable;

include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

/// Replaces all confusables characters in the string.
///
/// It first checks whether the input contains any confusable
/// characters in the first place. If you are certain it does,
/// you can call [`replace`] directly.
pub fn detect_replace(input: &str) -> Cow<'_, str> {
  if input.contains_confusable() {
    Cow::Owned(replace(input))
  } else {
    Cow::Borrowed(input)
  }
}

/// Replaces all confusable characters.
pub fn replace(input: &str) -> String {
  // Create a string buffer with room for more chars than the initial string
  // since some confusables map to more than one char.
  let mut output = String::with_capacity(input.len() * 2);

  input.chars().for_each(|c| {
    if let Some(s) = KEYWORDS.get(&c) {
      output.push_str(s);
    } else {
      output.push(c);
    }
  });

  output
}

#[cfg(test)]
mod tests {
  use crate::{confusable::Confusable, detect_replace};

  const DATA: [(&str, &str); 5] = [
    ("ǉeto", "ljeto"),
    ("pаypаl", "paypal"),
    ("ѕсоре", "scope"),
    ("𝓗℮𝐥1೦", "Hello"),
    ("m", "rn"),
  ];

  #[test]
  fn detect_replace_test() {
    for (input, output) in DATA {
      let res = detect_replace(input);

      assert_eq!(res, output);
    }
  }

  #[test]
  fn equality_test() {
    for (input, output) in DATA {
      assert!(input.contains_confusable());
      assert!(!output.contains_confusable());

      // Left "simplifies" to right
      assert!(input.is_confusable_with(output));
      // Right does not "simplify" to left
      assert!(!output.is_confusable_with(input));
    }
  }
}
