use diesel;
use diesel::PgConnection;
use diesel::prelude::*;
use ::models::{Player};

pub fn delete(conn: &PgConnection, player: Player) {
    diesel::delete(&player).execute(conn).expect("Error deleting Player");
}


pub fn delete_by_game(conn: &PgConnection, game_id_query: i32) {
    use ::schema::players::dsl::*;

    diesel::delete(players.filter(game_id.eq(game_id_query)))
        .execute(conn)
        .expect("Error deleting Player");
}
