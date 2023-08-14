#![allow(clippy::unwrap_used)]

use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

fn main() {
  // Run the build script when either this or the data file change
  println!("cargo:rerun-if-changed=build.rs");
  println!("cargo:rerun-if-changed=./data/confusables.txt");

  let path = Path::new(&env::var("OUT_DIR").unwrap()).join("codegen.rs");
  let mut file = BufWriter::new(File::create(path).unwrap());

  let mut map = phf_codegen::Map::<char>::new();

  let data = include_str!("./data/confusables.txt")
    .lines()
    .filter(|l| !l.starts_with('#') && !l.is_empty())
    .map(|l| l.split(" ;	").collect::<Vec<_>>())
    .map(|l| (l[0], l[1]))
    .map(|(source, target)| {
      let source = source.replace('\n', "");
      let target = target.replace('\n', "");

      let parsed_source = u32::from_str_radix(&source, 16)
        .unwrap_or_else(|_| panic!("'{source}' is not a valid hex number"));

      let parsed_source = char::from_u32(parsed_source).unwrap();

      let parsed_target = target
        .split_whitespace()
        .map(|el| {
          u32::from_str_radix(el, 16)
            .unwrap_or_else(|_| panic!("'{target}' is not a valid hex number"))
        })
        .map(|c| char::from_u32(c).unwrap_or_else(|| panic!("'{c}' is not a valid character")))
        .collect::<String>();

      (parsed_source, parsed_target)
    });

  for (source, target) in data {
    let _ = map.entry(source, &format!("r\"{target}\""));
  }

  write!(
    &mut file,
    "#[allow(clippy::unicode_not_nfc, clippy::unreadable_literal)]
    static KEYWORDS: phf::Map<char, &'static str> = {};",
    map.build()
  )
  .unwrap();
}
