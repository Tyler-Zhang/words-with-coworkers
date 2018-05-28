use super::{SlackCommand, SlackResponse};
use diesel::PgConnection;
use regex::Regex;
use services::{game_services, player_services};
use operations::game_operations;
use std;
use super::super::state::dictionary::ScrabbleDictionary;

#[derive(Debug)]
pub struct PlayWordParams {
    pub word: String,
    pub row: i32,
    pub col: i32,
    pub horizontal: bool,
}

pub fn play(command: &SlackCommand, db: &PgConnection, dict: &ScrabbleDictionary) -> Result<SlackResponse, String> {
    let game = game_services::get_by_channel_id(db, &command.channel_id);

    if game.is_none() {
        return Err(String::from("There is no game in this channel"));
    }

    let mut game = game.unwrap();
    let player = player_services::get_player_from_game(db, &game, &command.user_id);

    if (player.is_none()) {
        return Err(String::from("You are not participating in the game"));
    }

    let mut player = player.unwrap();

    let play_move = text_to_play_word_param(&command.text)?;

    println!("{:#?}", play_move);

    game_operations::gameplay::execute_move(&mut player, &mut game, &play_move, dict)?;

    game_services::update(db, &game);
    player_services::update(db, &player);

    Ok(SlackResponse {
        text: "hello".to_string(),
        response_type: Some("ephemeral".to_string()),
    })
}

pub fn text_to_play_word_param(text: &str) -> Result<PlayWordParams, String> {
    let re = Regex::new(r"play (?P<word>\w+) (?P<row>\d+):(?P<col>\d+) (?P<dir>\w+)").unwrap();

    let caps = re
        .captures(text)
        .ok_or("Your command is malformatted".to_string())?;

    Ok(PlayWordParams {
        word: caps.name("word").unwrap().to_owned(),
        row: caps.name("row").unwrap().parse::<i32>().unwrap(),
        col: caps.name("col").unwrap().parse::<i32>().unwrap(),
        horizontal: caps.name("dir").unwrap() == "right",
    })
}
