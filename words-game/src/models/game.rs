use super::super::error::*;
use super::board::{Board, OverlaidWord, BoardWithOverlay};
use super::player::Player;
use super::direction::*;
use super::tile::{TileBag, Tile};
use std::fmt;

#[derive(Debug)]
pub struct PlayWordResult {
    words: Vec<String>,
    score: u32,
}

#[derive(Debug, Clone)]
pub struct Game {
    pub board: Board,
    pub players: Vec<Player>,
    pub turn: u32,
    pub tile_bag: TileBag,
    pub has_word_been_played: bool,
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Board:")?;
        self.board.fmt(f)?;

        writeln!(f, "Players:")?;
        for (idx, ref player) in self.players.iter().enumerate() {
            writeln!(
                f,
                "{}: score: {}, pieces: {:?}",
                idx, player.score, player.hand
            )?;
        }

        writeln!(f, "Other:")?;
        writeln!(
            f,
            "turn: {}, has_word_been_played:{}, tile_bag: {}",
            self.turn, self.has_word_been_played, self.tile_bag
        )?;

        Ok(())
    }
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

impl Game {
    fn increment_turn(&mut self) {
        self.turn += 1;
    }

    pub fn get_current_player(&mut self) -> &mut Player {
        let player_idx = (self.turn as usize) % self.players.len();

        &mut self.players[player_idx]
    }

    pub fn new(player_count: usize) -> Game {
        let board = Board::new();
        let mut tile_bag = TileBag::new();
        let mut players = Vec::with_capacity(player_count);

        for _ in 0..player_count {
            players.push(Player::new(&mut tile_bag));
        }

        Game {
            board,
            players,
            turn: 0,
            tile_bag,
            has_word_been_played: false,
        }
    }

    pub fn play_word(&mut self, start: Point, dir: Direction, word: &str) -> Result<PlayWordResult> {
        let mut game = self.clone();

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
        let mut total_formed_words = Vec::<String>::new();

        let (word, score) = main_line_word.calculate_word_and_score()?;
        total_formed_words.push(word);
        total_score += score;

        for branching_word in branching_words.iter() {
            let (word, score) = branching_word.calculate_word_and_score()?;
            total_formed_words.push(word);
            total_score += score;
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

        *self = game;
        Ok(PlayWordResult{words: total_formed_words, score: total_score})
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
