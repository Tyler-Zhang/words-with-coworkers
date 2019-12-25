pub struct Player {
  pub score: u32,
  pub hand: Vec<Tile>,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Tile {
  Letter(char),
  Blank,
}

pub enum BoardCell {
  StartingSpot,
  Empty,
  DoubleLetter,
  TripleLetter,
  DoubleWord,
  TripleWord,
  Tile(Tile),
}

pub struct Board {
  pub cells: Vec<BoardCell>,
}

pub struct TileBag {
  pub tiles: Vec<Tile>,
}

pub struct Game {
  pub board: Board,
  pub players: Vec<Player>,
  pub turn: u32,
  pub tile_bag: TileBag,
}
