use diesel::PgConnection;
use diesel::prelude::*;
use ::models::{Player, Game};

pub fn get_players_from_game(conn: &PgConnection, game: &Game) -> Vec<Player> {
    use ::schema::players::dsl::*;

    Player::belonging_to(game).order(id).get_results(conn).expect("Could not load players")
}

pub fn get_player_from_game(conn: &PgConnection, game: &Game, slack_id_query: &str) -> Option<Player> {
    use ::schema::players::dsl::*;

    let res: Vec<Player> = Player::belonging_to(game)
        .filter(slack_id.eq(slack_id_query))
        .get_results(conn)
        .expect("Could not load player");

    return if res.len() == 0 { None } else { Some(res[0].clone()) };
}
