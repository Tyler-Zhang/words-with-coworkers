use diesel;
use diesel::PgConnection;
use diesel::prelude::*;
use ::models::{Game};

pub fn update(conn: &PgConnection, game: &Game) -> Game {
    diesel::update(game)
        .set(game)
        .get_result(conn)
        .expect("Could not update game")
}
