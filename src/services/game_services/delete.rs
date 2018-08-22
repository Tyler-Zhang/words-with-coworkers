use diesel;
use diesel::PgConnection;
use diesel::prelude::*;
use ::models::{Game};

pub fn delete(conn: &PgConnection, game: Game) -> Result<(), String> {
    let delete_count = diesel::delete(&game)
        .execute(conn)
        .or(Err(format!("Could not delete game")))?;

    if delete_count == 0 {
        Err(format!("No game to delete"))
    } else {
        Ok(())
    }
}
