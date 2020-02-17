use super::tile::Tile;

/**
 * Represents how the cell on the board affects the scoring of the final word
 */
#[derive(Debug)]
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
#[derive(Debug)]
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

impl Board {
  pub fn new() -> Board {

    let mut cells = Vec::new();

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
}
