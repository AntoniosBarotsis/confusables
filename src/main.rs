fn main() {
  // let text = "ﺔ";
  // let text = "\u{FE87}";
  let text = "\u{08F3}";
  let tmp = confusables::replace(text);

  println!("{text} -> {tmp}");
}
