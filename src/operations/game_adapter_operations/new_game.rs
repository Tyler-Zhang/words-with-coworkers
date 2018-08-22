use diesel::PgConnection;
use ::lib::scrabble;
use ::models::{GameAdapter, NewGame, NewPlayer};
use ::services::{player_services, game_services};

pub fn create_game(
    player_slack_ids: Vec<&str>,
    channel_id: &str,
    team_id: &str,
    conn: &PgConnection
) -> Result<GameAdapter, String> {
    let scrabble_game = scrabble::Game::new(player_slack_ids.len() as i32);

    // create game model
    let new_game = NewGame {
        board: &scrabble_game.board.to_string(),
        turn_count: scrabble_game.turn_count,
        pieces: &scrabble_game.pieces.clone(),
        channel_id,
        player_turn_id: None,
        team_id
    };

    let db_game = game_services::create_game(conn, &new_game)?;

    // Create the player models
    let db_players = {
        let new_players = player_slack_ids
            .iter()
            .enumerate()
            .map(|(index, player_slack_id)| NewPlayer {
                game_id: db_game.id,
                pieces: &scrabble_game.players[index].pieces,
                slack_id: player_slack_id,
                points: scrabble_game.players[index].score,
                team_id
            })
            .collect::<Vec<NewPlayer>>();

        player_services::create_players(conn, &new_players)?
    };


    let game_adapter = GameAdapter {
        scrabble_game,
        db_game,
        db_players
    };

    Ok(game_adapter)
}
