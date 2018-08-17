use std::collections::HashSet;
use super::super::config;
use super::{Board, Player, Word, Action};

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

    pub fn verify_word(&self, word: &Word) -> Result<(), String> {
        if word.start.x() > self.board.width || word.start.y() > self.board.height {
            return Err(format!("Starting position out of range"));
        }

        if word.direction_down && word.start.y() + (word.letters.len() as i32) > self.board.height ||
            !word.direction_down && word.start.x() + (word.letters.len() as i32) > self.board.width {
                return Err(format!("End of word out of range"));
        }

        Ok(())
    }

    fn play<'a>(&self, word: &'a str, start: (i32, i32), direction_down: bool, dict: &HashSet<String>) -> Result<Action, String> {
        let mut word = Word::new(word.to_owned(), start, direction_down);
        let mut action = Action::new(word);

        self.verify_word(&action.word)?;

        Ok(action)
    }
}

