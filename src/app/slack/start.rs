use diesel::PgConnection;
use super::{SlackCommand, SlackResponse};
use ::operations::game_adapter_operations;

pub fn start(command: &SlackCommand, db: &PgConnection) -> Result<SlackResponse, String> {
    // Check to see if there is already a game
    let existing_game = game_adapter_operations::get_game::get_game(&command.channel_id, db);

    if existing_game.is_ok() {
        return Ok(SlackResponse::new(
            format!("This channel is already playing a game!"),
            false
        ));
    }

    // Get all user's id's
    let mut ids = ::helpers::extract_user_ids(&command.text);

    if ids.len() == 0 {
        return Ok(SlackResponse::new (
            format!("You must tag the player you're trying to play with"),
            false
        ));
    }

    // Push sender id onto the vec
    ids.insert(0usize, &command.user_id);

    // Create the game
    let mut game_adapter = game_adapter_operations::new_game::create_game(
        ids,
        &command.channel_id,
        &command.team_id,
        db
    )?;

    let text = format!("\
    \nLets start a new game!\
    \n{}\
    \n>Use `/scrabbler hand` to see what pieces you have!
    \n>Use `/scrabbler play <word> <x:y> <right|down>` to play a word!
    ", game_adapter_operations::printing::format_game(&game_adapter, false));

    Ok(SlackResponse::new(text, true))
}
