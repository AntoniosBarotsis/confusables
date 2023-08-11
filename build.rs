#![allow(clippy::unwrap_used)]

use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

fn main() {
  let path = Path::new(&env::var("OUT_DIR").unwrap()).join("codegen.rs");
  let mut file = BufWriter::new(File::create(path).unwrap());

  let mut map = phf_codegen::Map::<String>::new();

  let data = include_str!("./data/confusablesSummary.txt");

  let data = data
    .lines()
    .filter(|l| l.starts_with("#	"))
    .map(|l| l.replace("#	", ""))
    .collect::<Vec<_>>();

  for el in data {
    let tmp = el + "\t";
    let chars = tmp.chars();

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

      if c == '\t' && !accumulator.is_empty() {
        let _ = map.entry(accumulator, &format!("r\"{first}\""));

        accumulator = String::new();
        first = String::new();
        continue;
      }

      accumulator += &(c.clone()).to_string();
    }
  }

  write!(
    &mut file,
    "static KEYWORDS: phf::Map<&'static str, &'static str> = {}",
    map.build()
  )
  .unwrap();

  writeln!(&mut file, ";").unwrap();
}
