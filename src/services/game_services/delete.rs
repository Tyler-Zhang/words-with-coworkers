use diesel;
use diesel::PgConnection;
use diesel::prelude::*;
use ::models::{Game};

pub fn delete(conn: &PgConnection, game: Game) {
    diesel::delete(&game).execute(conn).expect("Error deleting game");
}
