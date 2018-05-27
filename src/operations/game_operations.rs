use std::char;
use ::models::{Game, Player};

static STARTING_PIECES_COUNT: usize = 10;

pub fn give_initial_pieces(game: &mut Game, players: &mut Vec<Player>) {
    for player in players {
        let game_pieces_len = game.pieces.len();
        player.pieces = game.pieces.split_off(game_pieces_len - STARTING_PIECES_COUNT);
    }
}

pub fn game_board_to_str<'a>(game: &'a Game, use_emoji: bool) -> String {
    let board = &game.board;
    let width = game.board_width as usize;
    
    let mut printout = String::new();

    if use_emoji {
        for row in 0..board.len() / width  {
            for c in board[row*width..row*width + width].to_owned().into_bytes() {
                printout += &emoji_translator(c as char);
            }
            printout += &"\n";
        }
    } else {
        for row in 0..board.len() / width  {
            for c in board[row*width..row*width + width].to_owned().into_bytes() {
                printout += &format!("{}", c);
            }
            printout += &"\n";
        }
    }

    printout
}

pub fn translate_letters_to_emoji (text: &str) -> String {
    text.to_owned()
        .into_bytes()
        .into_iter()
        .map(|c| emoji_translator(c as char))
        .collect::<Vec<String>>()
        .join("")
}

fn emoji_translator(symbol: char) -> String {
    if symbol >= 'A' && symbol <= 'Z' {
        return format!(":scrabble-{}:", char::from_u32((symbol as u32) + 32).unwrap());
    } else {
        match symbol {
            '.' => String::from(":white_square:"),
            _ => format!("{}", symbol)
        }
    }
}
