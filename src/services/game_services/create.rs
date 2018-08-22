use diesel;
use diesel::PgConnection;
use diesel::prelude::*;
use ::models::{Game, NewGame};

pub fn create_game(conn: &PgConnection, new_game: &NewGame) -> Result<Game, String> {
    use ::schema::*;

    diesel::insert_into(games::table)
        .values(new_game)
        .get_result(conn)
        .or(Err(format!("Could not create game")))
}
