#![allow(clippy::unwrap_used)]

use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

use is_rtl::is_rtl;
use phf_codegen::Map;

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
    let first_char = el.chars().next().unwrap();

    if is_rtl(first_char) {
      parse(el.rsplit_terminator('\t'), &mut map);
    } else {
      parse(el.split_terminator('\t'), &mut map);
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

// The suggested lint fix doesn't compile 0_0
#[allow(single_use_lifetimes)]
fn parse<'a>(mut confusables: impl Iterator<Item = &'a str>, map: &mut Map<String>) {
  let first = confusables.next().unwrap();

  for c in confusables {
    let _ = map.entry(c.to_owned(), &format!("r\"{first}\""));
  }
}
