include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

fn main() {
  let c = "꓾";
  let tmp = KEYWORDS.get(c).expect("So confusing");
  println!("{c} -> {tmp}");
}
