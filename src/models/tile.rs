use super::super::error::{Error, Result};
use rand::prelude::*;
use std::fmt;

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

/**
 * A tile is a piece that is in the player's hand
 */
#[derive(Clone, PartialEq, Eq, Debug, Copy)]
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
                c => unreachable!("Trying to get string for invalid char {}", c),
            },
        }
    }
}

impl From<char> for Tile {
    fn from(c: char) -> Tile {
        match c {
            ' ' => Tile::Blank,
            c if ('A'..='Z').contains(&c) => Tile::Letter(c),
            c => unreachable!("Trying to make a tile with character {}", c),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TileBag {
    pub tiles: Vec<Tile>,
}

impl fmt::Display for TileBag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for tile in self.tiles.iter() {
            match tile {
                Tile::Blank => write!(f, "[]")?,
                Tile::Letter(letter) => write!(f, "{}", letter)?,
            }
        }
        Ok(())
    }
}

impl TileBag {
    pub fn new() -> TileBag {
        let mut tiles = Vec::with_capacity(100);

        // push_repeat(&mut tiles, Tile::Blank, 2);
        push_letter_tiles! {
          &mut tiles,
          'E' * 12,
          'A' * 9,
          'I' * 9,
          'O' * 8,
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

        let mut tile_bag = TileBag { tiles };
        tile_bag.shuffle();

        tile_bag
    }

    pub fn shuffle(&mut self) {
        self.tiles.shuffle(&mut thread_rng());
    }

    pub fn return_to(&mut self, tiles: &[Tile]) {
        for i in tiles {
            self.tiles.push(*i);
        }

        self.shuffle();
    }

    pub fn draw(&mut self, count: usize) -> Result<Vec<Tile>> {
        if count <= self.tiles.len() {
            Ok(self.tiles.drain(0..count).collect())
        } else {
            Err(Error::NotEnoughTiles.into())
        }
    }

    /**
     * Like draw but only draws up to the rest of the tiles in the bag
     */
    pub fn draw_upto(&mut self, count: usize) -> Vec<Tile> {
        self.draw(std::cmp::min(count, self.tiles.len())).unwrap()
    }
}
