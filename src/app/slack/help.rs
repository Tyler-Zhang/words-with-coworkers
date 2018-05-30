use super::{SlackCommand, SlackResponse};
use diesel::PgConnection;

pub fn help(_command: &SlackCommand, _db: &PgConnection) -> Result<SlackResponse, String> {
  let help_message = "\
  Here is how you use the scrabbler:

`/scrabbler help` - Brings up this help dialogue
`/scrabbler start [tags]` - Start a game with the person you're chatting to

in game:
`/scrabbler board` - show the state of the board
`/scrabbler hand` - shows you your hand
`/scrabbler play <word> <x>:<y> <right|down>` - to play a word
`/scrabbler dict <word>` - to check if a word is valid
`/scrabbler quit` - Quit the current game
  ";


  Ok(SlackResponse { text: help_message.to_string(), response_type: Some("ephemeral".to_string()) })
}
