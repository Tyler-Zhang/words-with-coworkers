use super::tile::{Tile, TileBag};
use super::super::constants::HAND_SIZE;

/**
 * Represents a player in the game, a player does not have a unique id
 * and instead is kept track based on its index within the players vector
 */
#[derive(Debug)]
pub struct Player {
  pub score: u32,
  pub hand: Vec<Tile>,
}

impl Player {
  pub fn new(tile_bag: &mut TileBag) -> Player{
    Player {
      score: 0,
      hand: tile_bag.draw(HAND_SIZE).unwrap(),
    }
  }
}


#[cfg(test)]
mod tests{
  use super::*;

  #[test]
  fn hand_size() {
    let tb = &mut TileBag::new();
    let player = Player::new(tb);

    assert_eq!(player.hand.len(), HAND_SIZE);
  }
}
