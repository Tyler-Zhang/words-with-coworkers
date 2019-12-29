use rand::prelude::*;
use std::fmt;

/**
 * Represents a player in the game, a player does not have a unique id
 * and instead is kept track based on its index within the players vector
 */
pub struct Player {
  pub score: u32,
  pub hand: Vec<Tile>,
}

/**
 * A tile is a piece that is in the player's hand
 */
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Tile {
  Letter(char),
  Blank,
}

impl Tile {
  pub fn point_value(&self) -> u32 {
    match self {
      Self::Blank => 0,
      Self::Letter(letter) => match letter {
        'E' | 'A' | 'I' | 'O' | 'N' | 'R' | 'T' | 'L' | 'S' | 'U' => 1,
        'D' | 'G' => 2,
        'B' | 'C' | 'M' | 'P' => 3,
        'F' | 'H' | 'V' | 'W' | 'Y' => 4,
        'K' => 5,
        'J' | 'X' => 8,
        'Q' | 'Z' => 10,
        c => panic!(format!("Trying to get string for invalid char {}", c)),
      },
    }
  }
}

/**
 * Represents how the cell on the board affects the scoring of the final word
 */
pub struct BoardCellMultiplier {
  word: u32,
  letter: u32,
}

impl BoardCellMultiplier {
  pub fn create(word: u32, letter: u32) -> Self {
    BoardCellMultiplier {
      word: word,
      letter: letter,
    }
  }
}

/**
 * Represents the state of a cell on the board
 */
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
      Self::DoubleLetter => BoardCellMultiplier::create(1, 2),
      Self::TripleLetter => BoardCellMultiplier::create(1, 3),
      Self::DoubleWord => BoardCellMultiplier::create(2, 1),
      Self::TripleWord => BoardCellMultiplier::create(3, 1),
      _ => BoardCellMultiplier::create(1, 1),
    }
  }
}

pub struct Board {
  pub cells: Vec<BoardCell>,
}

pub struct NotEnoughTilesError;

impl fmt::Debug for NotEnoughTilesError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "There's not enough tiles")
  }
}

impl fmt::Display for NotEnoughTilesError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "There's not enough tiles")
  }
}

pub struct TileBag {
  pub tiles: Vec<Tile>,
}

impl TileBag {
  pub fn shuffle(&mut self) {
    self.tiles.shuffle(&mut thread_rng());
  }

  pub fn draw(&mut self, count: u32) -> Result<Vec<Tile>, String> {
    let count = count as usize;

    if count <= self.tiles.len() {
      Ok(self.tiles.drain(0..count).collect())
    } else {
      Err("Not enough tiles in the bag".to_string())
    }
  }
}

pub struct Game {
  pub board: Board,
  pub players: Vec<Player>,
  pub turn: u32,
  pub tile_bag: TileBag,
}

impl Game {
  pub fn get_current_player<'a>(&'a mut self) -> &'a mut Player {
    let player_idx = (self.turn as usize) % self.players.len();

    &mut self.players[player_idx]
  }
}
