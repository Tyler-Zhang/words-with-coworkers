use diesel::PgConnection;
use super::{SlackCommand, SlackResponse};
use regex::Regex;
use ::services::{game_services, player_services};
use rand::{thread_rng, Rng};
use ::operations::{game_operations};

pub fn hand(command: &SlackCommand, db: &PgConnection) -> Result<SlackResponse, String> {
    let game = game_services::get_by_channel_id(db, &command.channel_id)?;

    let player = player_services::get_player_from_game(db, &game, &command.user_id);

    if player.is_none() {
        return Ok(SlackResponse::new("You are not participating in the game".to_string(), false));
    }

    let mut player = player.unwrap();

    if should_scramble(command.text.as_str())? {
        // Scramble the hand's pieces
        let mut shuffled: Vec<u8> = player.pieces.into_bytes();
        thread_rng().shuffle(&mut shuffled);

        player.pieces = String::from_utf8(shuffled).expect("Shuffle pieces");
        player_services::update(db, &player);
    }

    Ok(SlackResponse::new(
        format!("Your letters are: {}", game_operations::printing::translate_letters_to_emoji(&player.pieces)),
        false
    ))
}

pub fn should_scramble(text: &str) -> Result<bool, String> {
    let re = Regex::new(r"hand *(?P<scramble>scramble)?").unwrap();

    let caps = re.captures(text)
        .ok_or("Your command is malformatted")?;

    return Ok(caps.name("scramble").is_some());
}
