use std::char;
use ::models::{Game, Player};

static STARTING_PIECES_COUNT: usize = 10;

pub fn give_initial_pieces(game: &mut Game, players: &mut Vec<Player>) {
    for player in players {
        let game_pieces_len = game.pieces.len();
        player.pieces = game.pieces.split_off(game_pieces_len - STARTING_PIECES_COUNT);
    }
}
