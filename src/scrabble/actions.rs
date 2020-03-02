use super::constants::DICTIONARY;
use super::error::{Error, Result};
use super::models::Game;
use super::models::{Direction, Point};

pub struct ActionResult {}

pub fn play_word(game: &mut Game, start: &Point, direction: &Direction, word: &str) -> Result<()> {
  if !DICTIONARY.contains(word) {
    return Err(Error::BadAction(format!("Word is not valid")).into());
  }

  return Ok(());
}

pub fn skip_turn(game: &mut Game) {}

pub fn swap_pieces(game: &mut Game) {}
