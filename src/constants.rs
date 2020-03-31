use std::collections::HashSet;
use std::str;

pub static HAND_SIZE: usize = 10;

/*
  The Map of the default board
    . - Empty piece
    3 - Triple word
    2 - Double word
    @ - Double letter
    # - Triple letter
    + - Starting spot
*/
pub static BOARD: &'static str = "\
  3..@...3...@..3\
  .2...#...#...2.\
  ..2...@.@...2..\
  @..2...@...2..@\
  ....2.....2....\
  .#...#...#...#.\
  ..@...@.@...@..\
  3..@...+...@..3\
  ..@...@.@...@..\
  .#...#...#...#.\
  ....2.....2....\
  @..2...@...2..@\
  ..2...@.@...2..\
  .2...#...#...2.\
  3..@...3...@..3";

pub static BOARD_SIZE: u32 = 15;

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
