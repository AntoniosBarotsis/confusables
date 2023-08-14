include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

fn main() {
  // let c = 'ﺔ';
  // let c = '\u{FE87}';
  let c = '\u{08F3}';
  let tmp = KEYWORDS.get(&c).expect("So confusing");

  println!("{c} -> {tmp}");
}
