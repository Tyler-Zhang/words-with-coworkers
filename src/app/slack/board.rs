use diesel::PgConnection;
use super::{SlackCommand, SlackResponse};
use ::services::{game_services, player_services};
use ::operations::{game_operations};
use ::models::Player;

pub fn board(command: &SlackCommand, db: &PgConnection) -> Result<SlackResponse, String> {
    let game = game_services::get_by_channel_id(db, &command.channel_id);

    if game.is_none() {
        return Ok(SlackResponse::new("There is currently no game in this channel".to_string(), false));
    }

    let game = game.unwrap();
    let players = player_services::get_players_from_game(db, &game);
    
    let text = format!("\
    \nCurrent Game!\
    \n{}\
    ", game_operations::printing::format_game_state((&game, &players), true));

    Ok(SlackResponse::new(text, false))
}
