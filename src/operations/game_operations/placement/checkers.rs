use app::state::dictionary::ScrabbleDictionary;
use std:: collections::HashMap;
use ::models::{Game};
use ::app::slack::play::PlayWordParams;
use super::helpers::*;


pub fn check_placeable (board: &Vec<Vec<char>>, play: &PlayWordParams, pieces: &str) -> Result<(), String> {
    let mut needed_letters: HashMap<char, i32> = HashMap::new();

    for i in 0..play.word.len() {
        let row: usize = if play.horizontal { play.row as usize } else { play.row as usize + i };
        let col: usize = if play.horizontal { play.col as usize + i } else { play.col as usize };

        let board_letter = board[row][col];
        let word_letter = play.word[i..].chars().next().unwrap();
        if is_char_letter(board_letter) {
            // If the tile on the board is a letter, but the the same one, then there's a collision
            if board_letter != word_letter {
                return Err(String::from("Could not place, word collision"));
            }
        } else {
            *needed_letters.entry(word_letter).or_insert(0) += 1;
        }
    }

    for c in pieces.chars() {
        if needed_letters.contains_key(&c) {
            *needed_letters.entry(c).or_insert(-1) -= 1;
        }
    }


    for (_key, value) in needed_letters.iter() {
        if *value > 0 {
            return Err(String::from("You do not have enough pieces to do this"));
        }
    }

    Ok(())
}

pub fn check_parameter_valid(game: &Game, play: &PlayWordParams) -> Result<(), String> {
    if play.col < 0 || play.col >= game.board_width ||
        play.row < 0 || play.row >= game.board_height ||
        (play.horizontal && play.col + play.word.len() as i32 > game.board_width) ||
        (!play.horizontal && play.row + play.word.len() as i32 > game.board_height)
    {
        return Err(String::from("Row or Col is out of range"));
    }

    for c in play.word.chars() {
        if !is_char_letter(c) {
            return Err(String::from("The word can only contain letters A-Z"));
        }
    }

    Ok(())
}

pub fn check_no_extending_characters(board: &Vec<Vec<char>>, play: &PlayWordParams) -> Result<(), String> {
    if play.horizontal {
        let left = get_char_from_vec(board, play.col - 1, play.row);
        let right = get_char_from_vec(board, play.col + play.word.len() as i32, play.row);

        if (left.is_some() && is_char_letter(left.unwrap())) ||
           (right.is_some() && is_char_letter(right.unwrap())) {
               return Err(String::from("There are extending characters, please specify the whole"));
        }
    } else {
        let up = get_char_from_vec(board, play.col, play.row - 1);
        let down = get_char_from_vec(board, play.col, play.row + play.word.len() as i32);

        if (up.is_some() && is_char_letter(up.unwrap())) ||
           (down.is_some() && is_char_letter(down.unwrap())) {
               return Err(String::from("There are extending characters, please specify the whole"));
        }
    }

    Ok(())
}

pub fn check_valid_word(word: &str, dict: &ScrabbleDictionary) -> Result<(), String> {
    if !dict.is_word_valid(word) {
        return Err(format!("{} is not a valid word", word));
    }

    Ok(())
}

#[inline]
pub fn is_char_letter(c: char) -> bool {
    c >= 'A' && c <= 'Z'
}
