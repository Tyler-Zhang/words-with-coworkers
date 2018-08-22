use diesel::PgConnection;
use super::{SlackCommand, SlackResponse};
use ::operations::game_adapter_operations;

pub fn board(command: &SlackCommand, db: &PgConnection) -> Result<SlackResponse, String> {
    let game_adapter = game_adapter_operations::get_game::get_game(&command.channel_id, db)?;

    let text = format!("{}", game_adapter_operations::printing::format_game(&game_adapter, true));

    Ok(SlackResponse::new(text, false))
}
