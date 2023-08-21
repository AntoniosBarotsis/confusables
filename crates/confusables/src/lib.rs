use std::borrow::Cow;

include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

/// Returns [`true`] if the left "potentially confusable" string
/// can be confused with the right "non confusable" string.
/// 
/// ## Examples
/// 
/// ```
/// # use confusables::are_confusable;
/// let data = [
///   ("ǉeto", "ljeto"),
///   ("pаypаl", "paypal"),
///   ("ѕсоре", "scope"),
///   ("𝓗℮𝐥1೦", "Hello"),
///   ("m", "rn"),
/// ];
/// 
/// for (left, right) in data {
///   // Left "simplifies" to right
///   assert!(are_confusable(left, right));
/// 
///   // Right does *not* "simplify" to left
///   assert!(!are_confusable(right, left));
/// }
/// ```
pub fn are_confusable(left: &str, right: &str) -> bool {
  let left = detect_replace(left);

  left == right
}

/// Checks if the input contains any confusable characters.
pub fn is_confusable(input: &str) -> bool {
  input.chars().any(|c| KEYWORDS.get(&c).is_some())
}

/// Replaces all confusables characters in the string. 
/// 
/// It first checks whether the input contains any confusable
/// characters in the first place. If you are certain it does,
/// you can call [`replace`] directly.
pub fn detect_replace(input: &str) -> Cow<'_, str> {
  if is_confusable(input) {
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
  use crate::{detect_replace, are_confusable};

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

      assert_eq!(&res, output);
    }
  }
  
  #[test]
  fn equality_test() {
    for (input, output) in DATA {
      assert!(are_confusable(input, output));  // Left "simplifies" to right
      assert!(!are_confusable(output, input)); // Right does not "simplify" to left
    }
  }
}
