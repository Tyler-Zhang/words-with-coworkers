use super::{SlackCommand, SlackResponse};
use diesel::PgConnection;

pub fn start(command: &SlackCommand, db: &PgConnection) -> SlackResponse {
  let help_message = "\
  Here is how you use the scrabbler:

/scrabbler help - Brings up this help dialogue
/scrabbler start - Start a game with the person you're chatting to
/scrabbler quit - Quit the current game
  ";


  SlackResponse { text: help_message.to_string(), response_type: Some("ephemeral".to_string()) }
}
