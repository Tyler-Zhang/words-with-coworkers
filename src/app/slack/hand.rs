use diesel::PgConnection;
use super::{SlackCommand, SlackResponse};
use regex::Regex;
use ::services::player_services;
use rand::{thread_rng, Rng};
use ::operations::game_adapter_operations;
use ::lib::slack;

pub fn hand(command: &SlackCommand, db: &PgConnection) -> Result<SlackResponse, String> {
    let mut game_adapter = game_adapter_operations::get_game::get_game(&command.channel_id, db)?;
    let player = game_adapter.get_player_by_user_id_mut(&command.user_id);

    if player.is_none() {
        return Ok(SlackResponse::new("You are not participating in the game".to_string(), false));
    }

    let mut player = player.unwrap();

    if should_scramble(command.text.as_str())? {
        // Scramble the hand's pieces
        let mut shuffled: Vec<u8> = player.pieces.clone().into_bytes();
        thread_rng().shuffle(&mut shuffled);

        player.pieces = String::from_utf8(shuffled).expect("Shuffle pieces");
        player_services::update(db, &player)?;
    }

    Ok(SlackResponse::new(
        format!("Your letters are: {}", slack::emoji::str_to_emoji_string(&player.pieces)),
        false
    ))
}

pub fn should_scramble(text: &str) -> Result<bool, String> {
    let re = Regex::new(r"hand *(?P<scramble>scramble)?").unwrap();

    let caps = re.captures(text)
        .ok_or("Your command is malformatted")?;

    return Ok(caps.name("scramble").is_some());
}
