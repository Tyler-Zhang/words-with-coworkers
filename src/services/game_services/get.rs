use diesel::PgConnection;
use diesel::prelude::*;
use ::models::{Game};

pub fn get_by_channel_id(conn: &PgConnection, channel_id_query: &str) -> Result<Game, String> {
    use ::schema::games::dsl::*;

    let mut matched_games = games.filter(channel_id.eq(channel_id_query))
        .limit(1)
        .load::<Game>(conn)
        .expect("Error loading games");

    if matched_games.is_empty() {
        Err(format!("Game not found"))
    } else {
        Ok(matched_games.remove(0))
    }
}
