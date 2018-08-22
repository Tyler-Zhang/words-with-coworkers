use diesel;
use diesel::{Connection, PgConnection};
use ::models::GameAdapter;
use ::services::{player_services, game_services};

pub fn update_game(game_adapter: &GameAdapter, conn: &PgConnection) -> Result<(), String> {
    conn.transaction::<_,diesel::result::Error,_>(|| {
        game_services::update(conn, &game_adapter.db_game)
            .or(Err(diesel::result::Error::NotFound))?;

        for db_player in game_adapter.db_players.iter() {
            player_services::update(conn, db_player)
                .or(Err(diesel::result::Error::NotFound))?;
        }

        Ok(())
    }).or(Err(format!("Transaction failed: could not update game")))
}
