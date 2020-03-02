use super::super::error::{Error, Result};
use super::tile::Tile;
use super::{Direction, Point};

/**
 * Represents how the cell on the board affects the scoring of the final word
 */
#[derive(Debug, PartialEq)]
pub struct BoardCellMultiplier {
  word: u32,
  letter: u32,
}

impl BoardCellMultiplier {
  pub fn new(word: u32, letter: u32) -> Self {
    BoardCellMultiplier {
      word: word,
      letter: letter,
    }
  }
}

/**
 * Represents the state of a cell on the board
 */
#[derive(Debug, PartialEq)]
pub enum BoardCell {
  StartingSpot,
  Empty,
  DoubleLetter,
  TripleLetter,
  DoubleWord,
  TripleWord,
  Tile(Tile),
}

impl BoardCell {
  pub fn get_multiplier(&self) -> BoardCellMultiplier {
    match self {
      Self::DoubleLetter => BoardCellMultiplier::new(1, 2),
      Self::TripleLetter => BoardCellMultiplier::new(1, 3),
      Self::DoubleWord => BoardCellMultiplier::new(2, 1),
      Self::TripleWord => BoardCellMultiplier::new(3, 1),
      _ => BoardCellMultiplier::new(1, 1),
    }
  }
}

#[derive(Debug)]
pub struct Board {
  pub cells: Vec<BoardCell>,
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
static BOARD: &'static str = "\
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

static BOARD_SIZE: u32 = 15;

impl Board {
  pub fn new() -> Board {
    let mut cells = Vec::with_capacity((BOARD_SIZE * BOARD_SIZE) as usize);

    for c in BOARD.chars() {
      cells.push(match c {
        '.' => BoardCell::Empty,
        '3' => BoardCell::TripleWord,
        '2' => BoardCell::DoubleWord,
        '@' => BoardCell::DoubleLetter,
        '#' => BoardCell::TripleLetter,
        '+' => BoardCell::StartingSpot,
        _ => unreachable!("Tried to parse invalid character for board"),
      })
    }

    Board { cells }
  }

  #[inline]
  pub fn is_in_bounds(&self, x: u32, y: u32) -> bool {
    (0..BOARD_SIZE).contains(&x) && (0..BOARD_SIZE).contains(&y)
  }

  pub fn at(&self, x: u32, y: u32) -> Option<&BoardCell> {
    if !self.is_in_bounds(x, y) {
      return None;
    }

    self.cells.get((y * BOARD_SIZE + x) as usize)
  }

  pub fn set(&mut self, x: u32, y: u32, bc: BoardCell) -> Result<()> {
    if !self.is_in_bounds(x, y) {
      return Err(Error::BadAction(format!("Out of bounds")).into());
    }
    self.cells[(y * BOARD_SIZE + x) as usize] = bc;
    Ok(())
  }

  /**
   * Allows us to easily iterate over a line of the board
   *
   * The iterator function can control if it want's to continue iterator
   * by returning a result. An Err will immediately end the iteration
   */
  pub fn for_each(
    &self,
    start: &Point,
    direction: &Direction,
    len: u32,
    f: &dyn Fn(&Point, &BoardCell) -> std::result::Result<(), ()>,
  ) {
    let mut loc = (*start).clone();

    for _ in 0..len {
      if let Some(bc) = self.at(loc.x, loc.y) {
        if let Err(_) = f(&loc, bc) {
          return;
        }
      } else {
        return;
      }

      loc += direction;
    }
  }

  pub fn get_pieces_needed_for_place(
    &self,
    start: &Point,
    direction: &Direction,
    word: &str,
  ) -> Result<Vec<Tile>> {
    let mut needed_tiles = Vec::<Tile>::new();

    let mut loc = (*start).clone();

    for c in word.chars() {
      if let Some(&BoardCell::Tile(Tile::Letter(letter))) = self.at(loc.x, loc.y) {
        if letter != c {
          return Err(Error::BadAction(format!("This word cannot be placed")).into());
        }
      } else {
        needed_tiles.push(Tile::Letter(c));
      }

      loc += direction;
    }

    Ok(needed_tiles)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn at_no_tile_test() {
    let board = Board::new();

    assert_eq!(board.at(0, 0).unwrap(), &BoardCell::TripleWord);
    assert_eq!(board.at(3, 0).unwrap(), &BoardCell::DoubleLetter);
    assert_eq!(board.at(1, 1).unwrap(), &BoardCell::DoubleWord);
    assert_eq!(board.at(7, 7).unwrap(), &BoardCell::StartingSpot);
    assert_eq!(board.at(BOARD_SIZE, BOARD_SIZE), None);
  }

  #[test]
  fn set_and_get_tiles() {
    let mut board = Board::new();

    assert_eq!(
      board.set(0, 0, BoardCell::Tile(Tile::Letter('A'))).is_ok(),
      true
    );
    assert_eq!(board.at(0, 0).unwrap(), &BoardCell::Tile(Tile::Letter('A')));

    assert_eq!(
      board.set(BOARD_SIZE, BOARD_SIZE, BoardCell::Empty).is_err(),
      true
    );
  }

  #[test]
  fn pieces_for_place() {
    let mut board = Board::new();

    assert_eq!(
      board
        .get_pieces_needed_for_place(&Point::new(0, 0), &Direction::new(1, 0), "HI")
        .unwrap(),
      vec![Tile::Letter('H'), Tile::Letter('I')]
    );

    board.set(1, 0, BoardCell::Tile(Tile::Letter('E'))).unwrap();
    board.set(3, 0, BoardCell::Tile(Tile::Letter('L'))).unwrap();

    assert_eq!(
      board
        .get_pieces_needed_for_place(&Point::new(0, 0), &Direction::new(1, 0), "HELLO")
        .unwrap(),
      vec![Tile::Letter('H'), Tile::Letter('L'), Tile::Letter('O')]
    );
  }
}
