use super::{SlackCommand, SlackResponse};
use operations::game_operations::printing::translate_letters_to_emoji;
use diesel::PgConnection;

pub fn help(_command: &SlackCommand, _db: &PgConnection) -> Result<SlackResponse, String> {
  let help_message = format!("\
Here is how you use the scrabbler

>`/scrabbler help` - Brings up this help dialogue
>`/scrabbler start [tags]` - Start a game with the person you're chatting to

in game:
>`/scrabbler board` - Show the state of the board
>`/scrabbler hand` - Shows you your hand
>`/scrabbler hand scramble` - Scrambles the letters in your hand
>`/scrabbler play <word> <x>:<y> <right|down>` - To play a word
>`/scrabbler dict <word>` - To check if a word is valid
>`/scrabbler quit` - Quit the current game

>When in a game, you must specify the the entirety of the word in the direction that you are playing
>For example: if {} is on the board and you want to play {}, you must specify the entire word
>'fart' and not just 'art'
", translate_letters_to_emoji("F"), translate_letters_to_emoji("FART"));


  Ok(SlackResponse { text: help_message.to_string(), response_type: Some("ephemeral".to_string()) })
}
