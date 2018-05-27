use super::{SlackCommand, SlackResponse};
use ::services::{game_services, player_services};
use diesel::PgConnection;

pub fn play(command: &SlackCommand, db: &PgConnection) -> SlackResponse {

  SlackResponse { text: "hello".to_string(), response_type: Some("ephemeral".to_string()) }
}
