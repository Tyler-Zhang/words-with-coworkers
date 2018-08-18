use std::collections::HashSet;
use super::super::config;
use super::super::utils;
use super::{Board, Player, Word, Action, Tile, Line};

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

    pub fn get_player_on_turn(&mut self) -> &mut Player {
        let player_count = self.players.len() as i32;
        let player_idx = self.turn_count % player_count;

        &mut self.players[player_idx as usize]
    }

    pub fn verify_word_in_bounds(&self, word: &Word) -> Result<(), String> {
        if !self.board.check_in_bounds(word.start) {
            return Err(format!("Starting position out of range"));
        }

        if !self.board.check_in_bounds(word.get_end()) {
            return Err(format!("End of word out of range"));
        }

        Ok(())
    }

    pub fn letters_needed_for_place(&self, word: &Word) -> Result<Vec<char>, String> {
        let mut letters_needed: Vec<char> = Vec::new();
        let mut successful = true;
        let mut idx = 0;

        self.board.iterate_for(word.start, word.direction, word.letters.len() as i32, |tile: &Tile| {
            if !successful {
                return;
            }

            if let &Tile::Letter(c) = tile {
                if c != word.letters[idx] {
                    successful = false;
                }
            } else {
                letters_needed.push(word.letters[idx]);
            }

            idx += 1;
        });

        if successful {
            return Ok(letters_needed);
        } else {
            return Err(format!("This word conflicts with the letters already on the board"));
        }
    }

    // pub fn letters_needed_for_place(&self, word: &Word) ->

    fn play<'a>(&mut self, word: &'a str, start: (i32, i32), direction_down: bool, dict: &HashSet<String>) -> Result<Action, String> {
        let mut word = Word::new(word.to_owned(), start, direction_down);
        utils::word::extend_word(&self.board, &mut word);
        self.verify_word_in_bounds(&word)?;

        let letters_needed = self.letters_needed_for_place(&word)?;

        if letters_needed.len() == 0 {
            return Err(format!("You must extend the word"));
        }

        let starting_spot = self.board.get_starting_spot();

        if starting_spot.is_some() {
            let starting_spot = starting_spot.unwrap();

            if !Line::new(word.start, word.direction, word.letters.len() as i32).includes(&starting_spot) {
                return Err(format!("The first word must be placed on the starting spot"));
            }
        }

        let mut action = Action::new(word);

        {
            let mut player = self.get_player_on_turn();

            // Remove pieces from player
            player.remove_pieces(&letters_needed.iter().collect::<String>())?;

        }




        Ok(action)
    }
}

