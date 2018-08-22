use super::{SlackCommand, SlackResponse};
use diesel::PgConnection;
use regex::Regex;
use ::lib::slack;
use operations::game_adapter_operations;
use super::super::state::dictionary::ScrabbleDictionary;

#[derive(Debug)]
pub struct PlayWordParams {
    pub word: String,
    pub row: i32,
    pub col: i32,
    pub horizontal: bool,
}

pub fn play(command: &SlackCommand, db: &PgConnection, dict: &ScrabbleDictionary) -> Result<SlackResponse, String> {
    let mut game_adapter = game_adapter_operations::get_game::get_game(&command.channel_id, db)?;

    // Check if it's this player's turn
    let is_player_turn = game_adapter.get_player_on_turn().slack_id == command.user_id;
    if !is_player_turn {
        return Err(format!("It is not currently your turn"));
    }

    // Make the move
    let play_word_params = text_to_play_word_param(&command.text)?;
    let action_result = game_adapter.play_move(
        &play_word_params.word,
        (play_word_params.col, play_word_params.row),
        !play_word_params.horizontal,
        &dict.words
    )?;

    // Persist the new state
    game_adapter_operations::update_game::update_game(&game_adapter, db)?;

    let printout = format!(
        "\
        {}\
        \n{} Did:\
        \n>{}",
        game_adapter_operations::printing::format_game(&game_adapter, true),
        slack::tag::Tag(&command.user_id).to_string(),
        action_result.log.join("\n")
    );

    Ok(SlackResponse::new(printout, true))
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
