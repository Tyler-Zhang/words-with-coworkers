use std::collections::HashSet;
use super::super::config;
use super::{Board, Player, Action};

pub struct Game {
    pub board: Board,
    pub players: Vec<Player>,
    pub turn_count: u32,
    pub pieces: String
}

impl Game {
    pub fn new(player_count: u32) -> Self {
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

    pub fn get_player_turn_index(&self) -> u32 {
        let player_count = self.players.len() as u32;
        self.turn_count % player_count
    }

    pub fn verify_action(&self, action: &Action) -> Result<(), String> {
        if action.start.0 > self.board.width || action.start.1 > self.board.height {
            return Err(format!("Starting position out of range"));
        }

        if action.direction_down && action.start.1 + (action.word.len() as u32) > self.board.height ||
            !action.direction_down && action.start.0 + (action.word.len() as u32) > self.board.width {
                return Err(format!("End of word out of range"));
        }

        Ok(())
    }

    fn play<'a>(&self, word: &'a str, start: (u32, u32), direction_down: bool, dict: &HashSet<String>) -> Result<Action<'a>, String> {
        let mut action = Action::new(word, start, direction_down);
        self.verify_action(&action)?;

        Ok(action)
    }
}

