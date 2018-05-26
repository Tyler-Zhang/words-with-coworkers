use diesel::PgConnection;
use super::{SlackCommand, SlackResponse};
use ::operations::game_operations;

pub fn start(command: &SlackCommand, db: &PgConnection) -> SlackResponse {
    let game = game_operations::create_game(db, &command.channel_id);

    SlackResponse { text: format!("created new game with id {}", game.id), response_type: Some("ephemeral".to_string()) }
}
