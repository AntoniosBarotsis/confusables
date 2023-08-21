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
  use crate::detect_replace;

  #[test]
  fn test() {
    let data = vec![
      ("ǉeto", "ljeto"),
      ("pаypаl", "paypal"),
      ("ѕсоре", "scope"),
      ("𝓗℮𝐥1೦", "Hello"),
      ("m", "rn"),
    ];

    for (input, output) in data {
      let res = detect_replace(input);

      assert_eq!(&res, output);
    }
  }
}
