use std::collections::HashSet;
use super::super::config;
use super::{Board, Player, Word};

pub struct Game {
    pub board: Board,
    pub players: Vec<Player>,
    pub turn_count: i32,
    pub pieces: String
}

impl Game {
    pub fn new(player_count: i32) -> Self {
        let mut pieces = config::generate_default_pieces();

        let players = [0..player_count].iter()
            .map(|_| Player::new(pieces.split_off(config::PLAYER_HAND_PIECES_COUNT as usize)))
            .collect();

        Game {
            board: Board::new_default_board(),
            players: players,
            turn_count: 0,
            pieces: pieces
        }
    }

    pub fn get_player_turn_index(&self) -> i32 {
        let player_count = self.players.len() as i32;
        self.turn_count % player_count
    }

    pub fn verify_action(&self, word: &Word) -> Result<(), String> {
        if word.start.0 > self.board.width || word.start.1 > self.board.height {
            return Err(format!("Starting position out of range"));
        }

        if word.direction_down && word.start.1 + (word.letters.len() as i32) > self.board.height ||
            !word.direction_down && word.start.0 + (word.letters.len() as i32) > self.board.width {
                return Err(format!("End of word out of range"));
        }

        Ok(())
    }

    fn play<'a>(&self, word: &'a str, start: (i32, i32), direction_down: bool, dict: &HashSet<String>) -> Result<Word<'a>, String> {
        let mut action = Word::new(word, start, direction_down);
        self.verify_action(&action)?;

        Ok(action)
    }
}

