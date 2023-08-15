use std::borrow::Cow;

include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

fn main() {
  // let c = 'ﺔ';
  // let c = '\u{FE87}';
  let c = '\u{08F3}';
  let tmp = KEYWORDS.get(&c).expect("So confusing");

  println!("{c} -> {tmp}");
}

pub fn is_confusable(input: &str) -> bool {
  input.chars().any(|c| KEYWORDS.get(&c).is_some())
}

pub fn detect_replace(input: &str) -> Cow<'_, str> {
  if is_confusable(input) {
    Cow::Owned(replace(input))
  } else {
    Cow::Borrowed(input)
  }
}

pub fn replace(input: &str) -> String {
  let mut output = String::with_capacity(input.len());

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
    let data = vec![("ǉeto", "ljeto"), ("pаypаl", "paypal"), ("ѕсоре", "scope")];

    for (input, output) in data {
      let res = detect_replace(input);

      assert_eq!(&res, output);
    }
  }
}
