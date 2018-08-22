use diesel;
use diesel::PgConnection;
use diesel::prelude::*;
use ::models::{Game};

pub fn update(conn: &PgConnection, game: &Game) -> Result<Game, String> {
    diesel::update(game)
        .set(game)
        .get_result(conn)
        .or(Err(format!("Could not update game")))
}
