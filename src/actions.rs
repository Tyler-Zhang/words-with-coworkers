use super::error::{Error, Result};
use super::models::{
    BoardCell, BoardCellMultiplier, BoardWithOverlay, Direction, Game, OverlaidWord, Point, Tile,
};

pub struct ActionResult {}

pub fn start_game(player_count: u32) -> Game {
    Game::new(player_count as usize)
}

fn ensure_play_builds_on_other_words(
    original_word: &str,
    main_line_word: &OverlaidWord,
    branching_words: &[OverlaidWord],
    used_tiles: &[Tile],
) -> Result<()> {
    if used_tiles.is_empty() {
        Err(Error::NoLettersUsed.into())
    } else if main_line_word.len() == original_word.len()
        && branching_words.is_empty()
        && used_tiles.len() == original_word.len()
    {
        Err(Error::WordDoesNotIntersect.into())
    } else {
        Ok(())
    }
}

pub fn play_word(game: Game, start: Point, dir: Direction, word: &str) -> Result<Game> {
    let mut game = game;

    let board_overlay =
        BoardWithOverlay::try_overlay(game.board.clone(), start, dir, word)?;

    // Check validity of formed words, and add up the score
    let (main_line_word, branching_words) = board_overlay.get_formed_words();
    let needed_tiles = board_overlay.get_overlaid_letters();

    if !game.has_word_been_played {
        main_line_word.ensure_word_covering_starting_spot()?;
        game.has_word_been_played = true;
    } else {
        ensure_play_builds_on_other_words(
            word,
            &main_line_word,
            &branching_words,
            &needed_tiles,
        )?;
    }

    let mut total_score = 0u32;
    total_score += main_line_word.calculate_word_score()?;

    for branching_word in branching_words {
        total_score += branching_word.calculate_word_score()?;
    }

    // Check to make sure the player has the letters to make this play
    let new_tiles = game.tile_bag.draw_upto(needed_tiles.len());
    let player = game.get_current_player();

    player.remove_tiles_from_hand(&needed_tiles)?;
    player.add_tiles_to_hand(new_tiles);

    // Apply new score to the player
    player.add_score(total_score);

    // Apply new board state to the game
    game.board = board_overlay.apply_to_board();

    game.increment_turn();

    Ok(game)
}

pub fn skip_turn(_game: &mut Game) {}

pub fn swap_pieces(_game: &mut Game) {}
