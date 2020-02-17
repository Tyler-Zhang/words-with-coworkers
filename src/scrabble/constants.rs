use std::collections::HashSet;
use std::str;


pub static HAND_SIZE: usize = 10;

lazy_static! {
  pub static ref DICTIONARY: HashSet<&'static str> = {
    let mut dict = HashSet::new();

    let words = include_str!("dictionary.txt");

    for line in words.lines() {
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
