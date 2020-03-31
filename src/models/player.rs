use super::tile::{Tile, TileBag};
use super::super::constants::HAND_SIZE;
use super::super::error::{Error, Result};

fn remove_tiles(src: &Vec<Tile>, tiles: &[Tile]) -> Result<Vec<Tile>> {
  let mut rtn_tiles = src.clone();

  for tile in tiles {
    let idx = rtn_tiles
                .iter()
                .position(|x| x == tile)
                .ok_or(Error::NotEnoughTiles)?;

    rtn_tiles.remove(idx);
  }

  Ok(rtn_tiles)
}

/**
 * Represents a player in the game, a player does not have a unique id
 * and instead is kept track based on its index within the players vector
 */
#[derive(Debug, Clone)]
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

  pub fn add_score(&mut self, score: u32) {
    self.score += score
  }

  pub fn remove_tiles_from_hand(&mut self, tiles: &[Tile]) -> Result<()> {
    self.hand = remove_tiles(&self.hand, tiles)?;
    Ok(())
  }

  pub fn add_tiles_to_hand(&mut self, tiles: Vec<Tile>) {
    let mut tiles = tiles;

    self.hand.append(&mut tiles);
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
