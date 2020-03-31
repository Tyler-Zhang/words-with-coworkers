use super::constants::DICTIONARY;
use super::error::{Error, Result};
use super::models::{Game, BoardWithOverlay, Direction, Point, OverlaidWord, BoardCell, Tile, BoardCellMultiplier};

pub struct ActionResult {}

pub fn start_game(player_count: u32) -> Game {
  Game::new(player_count as usize)
}

fn calculate_word_score(word: &OverlaidWord) -> Result<u32> {
  let mut aggregate_word = Vec::<char>::with_capacity(word.len());

  let mut letter_score = 0;
  let mut word_multiplier = 1;

  for (bc, bottom_bc) in word {
    let curr_letter_val = match bc {
      BoardCell::Tile(tile) => {
        match tile {
          Tile::Letter(letter) => aggregate_word.push(*letter),
          _ => unreachable!()
        }
        tile.point_value()
      },
      _ => unreachable!()
    };

    if let Some(under_board_cell) = bottom_bc {
      let BoardCellMultiplier{ word: word_mult, letter: letter_mult }
            = under_board_cell.get_multiplier();

      letter_score += curr_letter_val * letter_mult;
      word_multiplier *= word_mult;
    } else {
      letter_score += curr_letter_val;
    }
  }

  let word = aggregate_word.into_iter().collect::<String>();
  if !DICTIONARY.contains(&word[..]) {
    Err(Error::InvalidWord(word).into())
  } else {
    Ok(letter_score * word_multiplier)
  }
}

fn ensure_word_covering_starting_spot(word: &OverlaidWord) -> Result<()> {
  for (_, under_bc) in word {
    if let Some(BoardCell::StartingSpot) = under_bc {
      return Ok(())
    }
  }

  Err(Error::StartingTileNotCovered.into())
}

fn ensure_play_builds_on_other_words(
  original_word: &str,
  main_line_word: &OverlaidWord,
  branching_words: &[OverlaidWord],
  used_tiles: &[Tile]) -> Result<()> {
    if used_tiles.len() == 0 {
      Err(Error::NoLettersUsed.into())
    } else if main_line_word.len() == original_word.len() &&
              branching_words.len() == 0 &&
              used_tiles.len() == original_word.len() {
      Err(Error::WordDoesNotIntersect.into())
    } else {
      Ok(())
    }
}

pub fn play_word(game: Game, start: &Point, dir: &Direction, word: &str) -> Result<Game> {
  let mut game = game;

  let board_overlay = BoardWithOverlay::try_overlay(
    game.board.clone(),
    start.clone(),
    dir.clone(),
    word
  )?;

  // Check validity of formed words, and add up the score
  let (main_line_word, branching_words) = board_overlay.get_formed_words();
  let needed_tiles = board_overlay.get_overlaid_letters();

  if !game.has_word_been_played {
    ensure_word_covering_starting_spot(&main_line_word)?;
    game.has_word_been_played = true;
  } else {
    ensure_play_builds_on_other_words(
      word,
      &main_line_word,
      &branching_words[..],
      &needed_tiles[..])?;
  }

  let mut total_score = 0u32;
  total_score += calculate_word_score(&main_line_word)?;

  for branching_word in branching_words {
    total_score += calculate_word_score(&branching_word)?;
  }

  // Check to make sure the player has the letters to make this play
  let new_tiles = game.tile_bag.draw_upto(needed_tiles.len());
  let player = game.get_current_player();

  player.remove_tiles_from_hand(&needed_tiles[..])?;
  player.add_tiles_to_hand(new_tiles);

  // Apply new score to the player
  player.add_score(total_score);

  // Apply new board state to the game
  game.board = board_overlay.apply_to_board();

  game.increment_turn();

  Ok(game)
}

pub fn skip_turn(game: &mut Game) {}

pub fn swap_pieces(game: &mut Game) {}
