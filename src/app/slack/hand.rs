use diesel::PgConnection;
use super::{SlackCommand, SlackResponse};
use ::services::{game_services, player_services};
use ::operations::{game_operations};
use ::models::Player;

pub fn hand(command: &SlackCommand, db: &PgConnection) -> Result<SlackResponse, String> {
    let game = game_services::get_by_channel_id(db, &command.channel_id);

    if game.is_none() {
        return Ok(SlackResponse::new("There is currently no game in this channel".to_string(), false));
    }

    let game = game.unwrap();
    
    let player = player_services::get_player_from_game(db, &game, &command.user_id);

    if (player.is_none()) {
        return Ok(SlackResponse::new("You are not participating in the game".to_string(), false));
    }

    Ok(SlackResponse::new(
        format!("Your letters are: {}", game_operations::printing::translate_letters_to_emoji(&player.unwrap().pieces)), 
        false
    ))
}
