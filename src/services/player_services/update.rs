use diesel;
use diesel::PgConnection;
use diesel::prelude::*;
use ::models::{Player};

pub fn update(conn: &PgConnection, player: &Player) -> Result<Player, String> {
    diesel::update(player)
        .set(player)
        .get_result(conn)
        .or(Err(format!("Could not update player")))
}
