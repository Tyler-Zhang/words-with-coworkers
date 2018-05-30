use super::{SlackCommand, SlackResponse};
use diesel::PgConnection;
use regex::Regex;
use services::{game_services, player_services};
use operations::game_operations;
use super::super::state::dictionary::ScrabbleDictionary;

#[derive(Debug)]
pub struct PlayWordParams {
    pub word: String,
    pub row: i32,
    pub col: i32,
    pub horizontal: bool,
}

pub fn dict(command: &SlackCommand, _db: &PgConnection, dict: &ScrabbleDictionary) -> Result<SlackResponse, String> {
    let word = text_to_play_word_param(&command.text)?;

    if dict.is_word_valid(&word) {
        return Ok(SlackResponse::new(format!("{} is a word", word), false));
    }

    Ok(SlackResponse::new(format!("{} is not a word", word), false))
}

pub fn text_to_play_word_param(text: &str) -> Result<String, String> {
    let re = Regex::new(r"dict (?P<word>\w+)").unwrap();

    let caps = re
        .captures(text)
        .ok_or("Your command is malformatted".to_string())?;

    Ok(caps.name("word").unwrap().to_owned().to_uppercase())
}
