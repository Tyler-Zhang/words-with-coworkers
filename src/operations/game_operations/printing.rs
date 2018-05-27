use std::char;
use ::models::{Game, Player};

static STARTING_PIECES_COUNT: usize = 10;

pub fn give_initial_pieces(game: &mut Game, players: &mut Vec<Player>) {
    for player in players {
        let game_pieces_len = game.pieces.len();
        player.pieces = game.pieces.split_off(game_pieces_len - STARTING_PIECES_COUNT);
    }
}

pub fn format_game_state(state: (&Game, &Vec<Player>), include_players: bool) -> String {
    let mut current_player_turn_str = "";

    let player_turn_id = state.0.player_turn_id.unwrap();
    
    for player in state.1 {
        if player.id == player_turn_id {
            current_player_turn_str = &player.slack_id;
        }
    }

    format!("\
        It is currently <@{}>'s turn
        \n{}\
        \n{}\
        ", 
        current_player_turn_str, 
        game_board_to_str(state.0, true), 
        if include_players { players_to_str(state.1) } else { "".to_string() }
    )
}

fn players_to_str(players: &Vec<Player>) -> String {
    players.iter().map(|player| format!("<@{}> - {}", player.slack_id, player.points)).collect::<Vec<String>>().join("\n")
}

fn game_board_to_str<'a>(game: &'a Game, use_emoji: bool) -> String {
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
