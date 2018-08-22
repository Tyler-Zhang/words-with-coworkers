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

pub fn play(command: &SlackCommand, db: &PgConnection, dict: &ScrabbleDictionary) -> Result<SlackResponse, String> {
    Ok(SlackResponse::new(format!("test"), true))
}

pub fn text_to_play_word_param(text: &str) -> Result<PlayWordParams, String> {
    let re = Regex::new(r"play (?P<word>\w+) (?P<col>\d+):(?P<row>\d+) (?P<dir>down|right)").unwrap();

    let caps = re
        .captures(text)
        .ok_or("Your command is malformatted".to_string())?;

    Ok(PlayWordParams {
        word: caps.name("word").unwrap().as_str().to_owned().to_uppercase(),
        row: caps.name("row").unwrap().as_str().parse::<i32>().unwrap(),
        col: caps.name("col").unwrap().as_str().parse::<i32>().unwrap(),
        horizontal: caps.name("dir").unwrap().as_str() == "right",
    })
}
