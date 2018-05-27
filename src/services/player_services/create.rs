use diesel;
use diesel::PgConnection;
use diesel::prelude::*;
use ::models::{Player, NewPlayer};

pub fn create_player(conn: &PgConnection, game_id: i32, slack_id: &str, team_id: &str) -> Player {
    use ::schema::*;

    let new_player = build_player(game_id, slack_id, team_id);

    diesel::insert_into(players::table)
        .values(&new_player)
        .get_result(conn)
        .expect("Error saving new game")
}

pub fn build_player<'a>(game_id: i32, slack_id: &'a str, team_id: &'a str) -> NewPlayer<'a> {
    NewPlayer {
        game_id: game_id,
        pieces: "", // Start the player off with 0 pieces
        slack_id: slack_id,
        points: 0,
        team_id: team_id
    }
}
