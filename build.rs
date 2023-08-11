#![allow(clippy::unwrap_used)]

use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

fn main() {
  // Run the build script when either this or the data file change
  println!("cargo:rerun-if-changed=build.rs");
  println!("cargo:rerun-if-changed=./data/confusablesSummary.txt");

  let path = Path::new(&env::var("OUT_DIR").unwrap()).join("codegen.rs");
  let mut file = BufWriter::new(File::create(path).unwrap());

  let mut map = phf_codegen::Map::<String>::new();

  let data = include_str!("./data/confusablesSummary.txt");

  // All the data we need is in the `# ...` lines
  let data = data
    .lines()
    .filter(|l| l.starts_with("#	"))
    .map(|l| l.replace("#	", ""))
    .collect::<Vec<_>>();

  for el in data {
    // Add a tab at the end of the string so we always read until a tab
    let tmp = el + "\t";
    let chars = tmp.chars();

    // Some confusables are multiple characters so we need to accumulate them.
    // They are separated by tabs.
    let mut accumulator = String::new();
    let mut first = String::new();
    let mut first_parsed = false;

    for c in chars {
      if !first_parsed {
        if c == '\t' {
          first_parsed = true;
        } else {
          first += &c.to_string();
        }
        continue;
      }

      // Delimiter
      if c == '\t' {
        let _ = map.entry(accumulator, &format!("r\"{first}\""));

        accumulator = String::new();
        first = String::new();
        continue;
      }

      // Append
      accumulator += &(c.clone()).to_string();
    }
  }

  write!(
    &mut file,
    "#[allow(clippy::unicode_not_nfc, clippy::unreadable_literal)]
    static KEYWORDS: phf::Map<&'static str, &'static str> = {}",
    map.build()
  )
  .unwrap();

  writeln!(&mut file, ";").unwrap();
}
