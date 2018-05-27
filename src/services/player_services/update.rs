use diesel;
use diesel::PgConnection;
use diesel::prelude::*;
use ::models::{Player};

pub fn update(conn: &PgConnection, player: &Player) -> Player {
    diesel::update(player)
        .set(player)
        .get_result(conn)
        .expect("Could not update player")
}
