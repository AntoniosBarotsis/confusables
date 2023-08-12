//! This crate provides a simple [`is_rtl`] function that checks whether a character belongs to a
//! Right-To-Left alphabet or not.

/// This returns true if the character belongs in a right-to-left (RTL) alphabet
///
/// The unicode ranges used in this function were taken from
/// [here](https://www.unicodepedia.com/groups/) and the list of RTL alphabets was taken
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
pub const fn is_rtl(c: char) -> bool {
  match c {
    '\u{600}'..='\u{6FF}'|         // Arabic
    '\u{10840}'..='\u{1085F}' |    // Imperial Aramaic
    '\u{591}'..='\u{5F4}'|         // Hebrew
    '\u{103A0}'..='\u{103D5}' |    // Old Persian
    '\u{700}'..='\u{74F}' => true, // Syrian
    _ => false
  }
}
