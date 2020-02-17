use super::board::Board;
use super::tile::TileBag;
use super::player::Player;

#[derive(Debug)]
pub struct Game {
  pub board: Board,
  pub players: Vec<Player>,
  pub turn: u32,
  pub tile_bag: TileBag,
}

impl Game {
  pub fn new(player_count: usize) -> Game {
    let board = Board::new();
    let mut tile_bag = TileBag::new();
    let mut players = Vec::with_capacity(player_count);

    for _ in 0..player_count {
      players.push(Player::new(&mut tile_bag));
    }

    Game {
      board: board,
      players: players,
      turn: 0,
      tile_bag: tile_bag,
    }
  }

  pub fn get_current_player<'a>(&'a mut self) -> &'a mut Player {
    let player_idx = (self.turn as usize) % self.players.len();

    &mut self.players[player_idx]
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn player_count() {
    assert_eq!(Game::new(4).players.len(), 4);
    assert_eq!(Game::new(2).players.len(), 2);
  }

  #[test]
  fn board_size() {
    assert_eq!(Game::new(1).board.cells.len(), 225);
  }

  #[test]
  fn tile_bag_different() {
    assert_ne!(Game::new(1).tile_bag.tiles, Game::new(1).tile_bag.tiles);
  }
}
