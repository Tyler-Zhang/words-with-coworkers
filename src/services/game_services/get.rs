use diesel::PgConnection;
use diesel::prelude::*;
use ::models::{Game};

pub fn get_by_channel_id(conn: &PgConnection, channel_id_query: &str) -> Option<Game> {
    use ::schema::games::dsl::*;

    let matched_games = games.filter(channel_id.eq(channel_id_query))
        .limit(1)
        .load::<Game>(conn)
        .expect("Error loading games");
    
    if matched_games.is_empty() {
        None
    } else {
        Some(matched_games[0].clone())
    }
}
