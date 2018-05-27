use diesel::PgConnection;
use super::{SlackCommand, SlackResponse};
use ::services::{game_services, player_services};
use ::operations::{game_operations};
use ::models::Player;

pub fn start(command: &SlackCommand, db: &PgConnection) -> SlackResponse {
    // Check to see if there is already a game
    let existing_game = game_services::get_by_channel_id(db, &command.channel_id);

    if existing_game.is_some() {
        return SlackResponse::new(
            format!("This channel is already playing a game! id: {}", existing_game.unwrap().id), 
            false
        );
    }

    // Get all user's id's
    let mut ids = ::helpers::extract_user_ids(&command.text);
    
    if ids.len() != 1 {
        return SlackResponse::new (
            format!("You must tag the player you're trying to play with"),
            false
        );
    }

    // Push sender id onto the vec
    ids.push(&command.user_id);
    
    // Create the game
    let mut game = game_services::create_game(db, &command.channel_id, &command.team_id);

    // Create the player objects
    let mut players: Vec<Player> = ids.into_iter()
        .map(|id| player_services::create_player(db, game.id, &id, &command.team_id))
        .collect();
    
    // Set the initial player
    game.player_turn_id = Some(players[0].id);

    // Hand out inital game pieces
    game_operations::gameplay::give_initial_pieces(&mut game, &mut players);
    game_services::update(db, &game);
    players.iter().for_each(|player| { player_services::update(db, &player); });

    let text = format!("\
    \nLets start a new game!\
    \n{}\
    \nUse `/scrabbler hand` to see what pieces you have!
    \nUse `/scrabbler play <word> <row:col> <right | down>` to play a word!
    ", game_operations::printing::format_game_state((&game, &players), true));

    SlackResponse::new(text, true)
}
