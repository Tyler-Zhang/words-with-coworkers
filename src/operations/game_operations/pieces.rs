use std::{char, cmp};
use std::collections::HashMap;
use ::models::{Game, Player};

static NORMAL_PIECES_COUNT: usize = 10;

pub fn give_initial_pieces(game: &mut Game, players: &mut Vec<Player>) {
    for player in players {
        let game_pieces_len = game.pieces.len();
        player.pieces = game.pieces.split_off(game_pieces_len - NORMAL_PIECES_COUNT);
    }
}

pub fn remove_pieces(player: &mut Player, pieces: &str) -> Result<(), String>{
    let mut pieces_to_remove = HashMap::new();

    pieces.chars().for_each(|c| *pieces_to_remove.entry(c).or_insert(0) += 1);

    let new_pieces: String = player.pieces.chars().filter(|c| -> bool {
        if !pieces_to_remove.contains_key(c) {
            return true;
        }

        if pieces_to_remove[c] > 0 {
            *pieces_to_remove.get_mut(c).unwrap() -= 1;
            return false;
        }

        return true;
    }).collect::<String>();

    for (key, val) in pieces_to_remove.iter() {
        if *val > 0 {
            return Err(String::from("Dont have enough pieces to remove"));
        }
    }

    player.pieces = new_pieces;

    Ok(())
}

pub fn give_pieces(player: &mut Player, game: &mut Game) {
    let needed_pieces_count = NORMAL_PIECES_COUNT - player.pieces.len();

    let game_pieces_len = game.pieces.len();
    player.pieces += &game.pieces.split_off(cmp::max(game_pieces_len - needed_pieces_count, 0));
}
