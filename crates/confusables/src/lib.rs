#![doc = include_str!("../README.md")]

pub use confusable::Confusable;

mod confusable;

include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

#[cfg(test)]
mod tests {
  use crate::confusable::Confusable;

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
      let res = input.detect_replace_confusable();

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
