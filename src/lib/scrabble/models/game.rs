use std::collections::HashSet;
use std::cmp;
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
            .map(|_| Player::new(&pieces.split_off(config::PLAYER_HAND_PIECES_COUNT as usize)))
            .collect();

        Game {
            board: Board::new_default_board(),
            players: players,
            turn_count: 0,
            pieces: pieces
        }
    }

    pub fn hydrate(board: Board, players: Vec<Player>, turn_count: i32, pieces: &str) -> Self {
        Self {
            board,
            players,
            turn_count,
            pieces: pieces.to_string()
        }
    }

    pub fn get_player_turn_index(&self) -> i32 {
        let player_count = self.players.len() as i32;
        self.turn_count % player_count
    }

    pub fn get_player_on_turn(&mut self) -> &mut Player {
        &mut self.players[self.get_player_turn_index() as usize]
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

    pub fn verify_first_word_on_starting_tile(&self, word: &Word) -> Result<(), String> {
        let starting_spot = self.board.get_starting_spot();
        if starting_spot.is_some() {
            let starting_spot = starting_spot.unwrap();

            if !Line::new(word.start, word.direction, word.letters.len() as i32).includes(&starting_spot) {
                return Err(format!("The first word must be placed on the starting spot"));
            }
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

    pub fn get_score_from_place(&self, word: &Word) -> i32 {
        let mut idx: usize = 0;
        let mut total_score: i32 = 0;

        let mut score_mutliplier: i32 = 1;

        self.board.iterate_for(word.start, word.direction, word.letters.len() as i32, |tile: &Tile| {
            let letter_score = config::get_letter_score(word.letters[idx]);

            if let &Tile::Letter(c) = tile {
                assert_eq!(c, word.letters[idx]);
            }

            let (word_multiplier, letter_multiplier) = tile.get_multiplier();

            total_score += letter_score * letter_multiplier;
            score_mutliplier *= word_multiplier;

            idx += 1;
        });

        total_score * score_mutliplier
    }

    // Calculates the total score for playing a word including the perpendicular words
    pub fn get_scores_for_playing_word(&self, word: &Word) -> Vec<(i32, String)> {
        let mut scored_words: Vec<(i32, String)> = Vec::new();

        // Start with perpendicular words
        let mut idx: i32 = 0;

        self.board.iterate_for(word.start, word.direction, word.letters.len() as i32, |tile: &Tile|{
            // If the tile is a letter, then we should keep going since perpendicular words
            // were already there before
            if let &Tile::Letter(_) = tile {
            } else {
                let mut perpendicular_word = Word::new(
                    format!("{}", word.letters[idx as usize]),
                    Into::<(i32, i32)>::into(word.start + (word.direction * idx)),
                    !word.direction_down
                );

                utils::word::extend_word(&self.board, &mut perpendicular_word);

                if perpendicular_word.letters.len() > 1 {
                    scored_words.push((
                        self.get_score_from_place(&perpendicular_word),
                        perpendicular_word.letters.iter().collect::<String>()
                    ));
                }
            }

            idx += 1;
        });

        // The word itself
        scored_words.push((
            self.get_score_from_place(word),
            word.letters.iter().collect::<String>()
        ));

        scored_words
    }

    pub fn play(&mut self, word: &str, start: (i32, i32), direction_down: bool, dict: &HashSet<String>) -> Result<Action, String> {
        let mut word = Word::new(word.to_owned(), start, direction_down);
        utils::word::extend_word(&self.board, &mut word);

        self.verify_word_in_bounds(&word)?;
        self.verify_first_word_on_starting_tile(&word)?;


        let letters_needed = self.letters_needed_for_place(&word)?;

        if letters_needed.len() == 0 {
            return Err(format!("You must extend the word"));
        }

        let counted_words = self.get_scores_for_playing_word(&word);
        let mut action = Action::new(word);

        let mut total_points: i32 = 0;

        for (value, word) in counted_words {
            total_points += value;
            action.log.push(format!("{} points for playing {}", value, word));
        }

        if letters_needed.len() as i32 == config::BINGO_LETTERS_PLAYED {
            total_points += config::BINGO_POINT_VALUE;
            action.log.push(format!("Additional {} points for bingoing!", config::BINGO_LETTERS_PLAYED));
        }

        {
            let pieces_in_player_hand = {
                let mut player = self.get_player_on_turn();

                // Remove pieces from player
                player.remove_pieces(&letters_needed.iter().collect::<String>())?;

                // Give score to the player
                player.score += total_points;

                player.pieces.len()
            };

            // Give pieces to player
            let max_pieces_to_give = cmp::min(
                self.pieces.len(),
                (config::PLAYER_HAND_PIECES_COUNT as usize) - pieces_in_player_hand
            );

            let pieces_to_give = self.pieces.split_off(max_pieces_to_give);

            self.get_player_on_turn().give_pieces(&pieces_to_give);
        }

        Ok(action)
    }
}

