use super::{SlackCommand, SlackResponse};
use ::services::{game_services, player_services};
use diesel::PgConnection;

pub fn quit(command: &SlackCommand, db: &PgConnection) -> Result<SlackResponse, String> {
    let mut game = game_services::get_by_channel_id(db, &command.channel_id)?;

    game.player_turn_id = None;
    game_services::update(db, &game);

    player_services::delete_by_game(db, game.id);
    game_services::delete(db, game);

    Ok(SlackResponse::new(format!("<@{}> has ended the game", command.user_id) , true))
}
