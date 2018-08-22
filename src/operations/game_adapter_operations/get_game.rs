use diesel::PgConnection;
use ::models::GameAdapter;
use ::services::{player_services, game_services};

pub fn get_game(channel_id: &str, conn: &PgConnection) -> Result<GameAdapter, String> {
    let db_game = game_services::get_by_channel_id(conn, channel_id)?;
    let db_players = player_services::get_players_from_game(conn, &db_game)?;

    Ok(GameAdapter::hydrate(db_game, db_players))
}
