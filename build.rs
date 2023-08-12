#![allow(clippy::unwrap_used)]

use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

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
      yes(el.rsplit_terminator('\t'), &mut map);
    } else {
      yes(el.split_terminator('\t'), &mut map);
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
fn yes<'a>(mut confusables: impl Iterator<Item = &'a str>, map: &mut Map<String>) {
  let first = confusables.next().unwrap();

  for c in confusables {
    let _ = map.entry(c.to_owned(), &format!("r\"{first}\""));
  }
}

/// This returns true if the character belongs in a right-to-left (RTL) alphabet
///
/// The unicode ranges used in this function were taken from
/// [here](https://www.unicodepedia.com/groups/) and the list of RTL alphabets were taken
/// from [here](https://www.worldatlas.com/articles/which-languages-are-written-from-right-to-left.html).
///
/// ## Unicode Groups
///
/// Not all the RTL alphabets seem to be represented in the unicode standard so these are the ones
/// the code *actually* looks for:
///
/// - Arabic
/// - Imperial Aramaic
/// - Hebrew
/// - Old Persian
/// - Syrian
// TODO: Move this to a separate crate and publish it as it might actually be useful
const fn is_rtl(c: char) -> bool {
  match c {
    '\u{600}'..='\u{6FF}'|         // Arabic
    '\u{10840}'..='\u{1085F}' |    // Imperial Aramaic
    '\u{591}'..='\u{5F4}'|         // Hebrew
    '\u{103A0}'..='\u{103D5}' |    // Old Persian
    '\u{700}'..='\u{74F}' => true, // Syrian
    _ => false
  }
}
