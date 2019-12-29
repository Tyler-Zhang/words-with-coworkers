use super::models::*;

pub struct Point {
  x: u32,
  y: u32,
}

pub struct Direction {
  x: u32,
  y: u32,
}

pub fn play_word(game: &mut Game, start: &Point, direction: &Direction, word: String) {}

pub fn skip_turn(game: &mut Game) {}

pub fn swap_pieces(game: &mut Game) {}
