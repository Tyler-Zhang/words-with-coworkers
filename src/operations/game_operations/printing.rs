use std::char;
use ::models::{Game, Player};


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
        let mut header_numbers = String::from(":white_square:");
        for i in 0..game.board_width {
            header_numbers.push_str(&translate_number_to_emoji(i));
        }

        printout.push_str(&(header_numbers + &"\n"));

        for row in 0..board.len() / width  {
            printout.push_str(&translate_number_to_emoji(row as i32));
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

pub fn translate_number_to_emoji (num: i32) -> String {
    format!(":{}:", number_to_word(num % 10))
}

pub fn number_to_word (num: i32) -> String {
    match num {
        0 => format!("zero"),
        1 => format!("one"),
        2 => format!("two"),
        3 => format!("three"),
        4 => format!("four"),
        5 => format!("five"),
        6 => format!("six"),
        7 => format!("seven"),
        8 => format!("eight"),
        9 => format!("nine"),
        _ => panic!(format!("{} too high", num))
    }
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
            '.' => String::from(":scrabble_board:"),
            '2' => String::from(":scrabble_double_word:"),
            '3' => String::from(":scrabble_triple_word:"),
            '@' => String::from(":scrabble_double_letter:"),
            '#' => String::from(":scrabble_triple_letter:"),
            '+' => String::from(":scrabble_start:"),
            _ => format!("{}", symbol)
        }
    }
}
