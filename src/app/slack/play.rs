use super::{SlackCommand, SlackResponse};
use diesel::PgConnection;

pub fn play(command: &SlackCommand, db: &PgConnection) -> SlackResponse {

  SlackResponse { text: "hello".to_string(), response_type: Some("ephemeral".to_string()) }
}
