use super::{SlackCommand, SlackResponse};
use diesel::PgConnection;
use regex::Regex;
use super::super::state::dictionary::ScrabbleDictionary;

pub fn dict(command: &SlackCommand, _db: &PgConnection, dict: &ScrabbleDictionary) -> Result<SlackResponse, String> {
    let word = extract_text(&command.text)?;

    if dict.is_word_valid(&word) {
        return Ok(SlackResponse::new(format!("{} is a word", word), false));
    }

    Ok(SlackResponse::new(format!("{} is not a word", word), false))
}

pub fn extract_text(text: &str) -> Result<String, String> {
    let re = Regex::new(r"dict (?P<word>\w+)").unwrap();

    let caps = re
        .captures(text)
        .ok_or("Your command is malformatted".to_string())?;

    Ok(caps.name("word").unwrap().as_str().to_owned().to_uppercase())
}
