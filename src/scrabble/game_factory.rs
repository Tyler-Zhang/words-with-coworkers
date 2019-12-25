use super::models::{Board, BoardCell, Game, Player, Tile, TileBag};
use rand::prelude::*;

/**
 * This macro is used to add some syntactic sugar to when we're pushing
 * the letter tiles into our tile_bag.
 *
 * Variables that start with c refer to the character,
 * Variables that start with n refer to the count (number of times we insert)
 */
macro_rules! push_letter_tiles {
  ($vec:expr, $c:literal * $n:literal) => {
    push_repeat($vec, Tile::Letter($c), $n);
  };

  ($vec:expr, $c:literal * $n:literal, $($cr:literal * $nr:literal),+) => {{
    push_letter_tiles! {$vec, $c * $n}
    push_letter_tiles! {$vec, $($cr * $nr),+}
  }}
}

fn push_repeat<T: Clone>(vec: &mut Vec<T>, t: T, count: u32) {
  for _ in 0..count {
    vec.push(t.clone());
  }
}

fn starting_tiles() -> Vec<Tile> {
  let mut vec = Vec::new();

  push_repeat(&mut vec, Tile::Blank, 2);
  push_letter_tiles! {
    &mut vec,
    'E' * 12,
    'A' * 9,
    'I' * 9,
    '0' * 8,
    'N' * 6,
    'R' * 6,
    'T' * 6,
    'L' * 4,
    'S' * 4,
    'U' * 4,
    'D' * 4,
    'G' * 3,
    'B' * 2,
    'C' * 2,
    'M' * 2,
    'P' * 2,
    'F' * 2,
    'H' * 2,
    'V' * 2,
    'W' * 2,
    'Y' * 2,
    'K' * 1,
    'J' * 1,
    'X' * 1,
    'Q' * 1,
    'Z' * 1
  }

  vec
}

fn create_tile_bag() -> TileBag {
  let mut tiles = starting_tiles();
  tiles.shuffle(&mut thread_rng());
  TileBag { tiles: tiles }
}

/*
  The Map of the default board
    . - Empty piece
    3 - Triple word
    2 - Double word
    @ - Double letter
    # - Triple letter
    + - Starting spot
*/
fn default_board() -> String {
  "\
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
   3..@...3...@..3"
    .to_string()
}

fn create_board() -> Board {
  let mut cells = Vec::new();

  for c in default_board().chars() {
    cells.push(match c {
      '.' => BoardCell::Empty,
      '3' => BoardCell::TripleWord,
      '2' => BoardCell::DoubleWord,
      '@' => BoardCell::DoubleLetter,
      '#' => BoardCell::TripleLetter,
      '+' => BoardCell::StartingSpot,
      _ => panic!("Tried to parse invalid character for board"),
    })
  }

  Board { cells: cells }
}

fn create_player(tile_bag: &mut TileBag) -> Player {
  Player {
    score: 0,
    hand: Vec::new(),
  }
}

pub fn create_game(player_count: u32) -> Game {
  let board = create_board();
  let mut tile_bag = create_tile_bag();
  let mut players = Vec::new();

  for _ in 0..player_count {
    players.push(create_player(&mut tile_bag));
  }

  Game {
    board: board,
    players: players,
    turn: 0,
    tile_bag: tile_bag,
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn player_count() {
    assert_eq!(create_game(4).players.len(), 4);
    assert_eq!(create_game(2).players.len(), 2);
  }

  #[test]
  fn board_size() {
    assert_eq!(create_game(1).board.cells.len(), 225);
  }

  #[test]
  fn tile_bag_different() {
    assert_ne!(create_game(1).tile_bag.tiles, create_game(1).tile_bag.tiles);
  }
}
