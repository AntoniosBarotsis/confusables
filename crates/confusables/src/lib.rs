use clean_string::CleanString;

pub mod clean_string;

include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

/// Checks if the input contains any confusable characters.
pub fn is_confusable(input: &str) -> bool {
  input.chars().any(|c| KEYWORDS.get(&c).is_some())
}

/// Replaces all confusables characters in the string.
///
/// It first checks whether the input contains any confusable
/// characters in the first place. If you are certain it does,
/// you can call [`replace`] directly.
pub fn detect_replace(input: &str) -> CleanString {
  if is_confusable(input) {
    replace(input)
  } else {
    CleanString {
      inner: input.to_owned(),
    }
  }
}

/// Replaces all confusable characters.
pub fn replace(input: &str) -> CleanString {
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

  CleanString { inner: output }
}

#[cfg(test)]
mod tests {
  use crate::{detect_replace, replace};

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
      let input_replaced = replace(input);
      let output_replaced = replace(output);

      // Left "simplifies" to right
      assert!(input_replaced.is_confusable_with(output));
      // Right does not "simplify" to left
      assert!(!output_replaced.is_confusable_with(input));
    }
  }
}
