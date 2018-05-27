use super::{SlackCommand, SlackResponse};
use ::services::{game_services, player_services};
use diesel::PgConnection;

pub fn quit(command: &SlackCommand, db: &PgConnection) -> SlackResponse {
    let game = game_services::get_by_channel_id(db, &command.channel_id);

    if game.is_none() {
        return SlackResponse::new(String::from("There isn't a game in this channel"), false);
    }

    let mut game = game.unwrap();

    game.player_turn_id = None;
    game_services::update(db, &game);

    player_services::delete_by_game(db, game.id);
    game_services::delete(db, game);

    SlackResponse::new(format!("<@{}> has ended the game", command.user_id) , true)
}
