use diesel;
use diesel::PgConnection;
use diesel::prelude::*;
use ::models::{Player, NewPlayer};

pub fn create_players(conn: &PgConnection, players: &Vec<NewPlayer>) -> Result<Vec<Player>, String> {
    use ::schema::*;

    diesel::insert_into(players::table)
        .values(players)
        .get_results(conn)
        .or(Err(format!("Problem inserting players")))
}
