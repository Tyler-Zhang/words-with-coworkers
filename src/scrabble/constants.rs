use std::collections::HashSet;
use std::str;

pub static HAND_SIZE: u32 = 10;

lazy_static! {
  pub static ref DICTIONARY: HashSet<&'static str> = {
    let mut dict = HashSet::new();

    let dictionary_bytes = include_bytes!("dictionary.txt");
    let s = str::from_utf8(dictionary_bytes).unwrap();

    for line in s.lines() {
      dict.insert(line);
    }

    dict
  };
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn dictionary_has_word() {
    assert_eq!(DICTIONARY.contains("MOTORCYCLE"), true);
  }

  #[test]
  fn dictionary_no_word() {
    assert_eq!(DICTIONARY.contains("BLAHBLAHBLAH"), false);
  }
}
