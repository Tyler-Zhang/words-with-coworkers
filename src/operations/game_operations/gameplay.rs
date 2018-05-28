use std:: collections::HashMap;
use ::models::{Player, Game};
use ::app::slack::play::PlayWordParams;
use app::state::dictionary::ScrabbleDictionary;


/*
    This is definitely the hardest part of the entire project. Lets follow these steps:
    1. Turn the board into a 2d array so it's easier to work with
    2. Check to make sure that there are no extending pieces in either direction
        - Eg: the player must specify the entirety of the word in the direction that they choose
    3. Sum up the points for the main word (skip if it's one letter)
    4. Sum up the points for the words perpendicular to the main word

    ** For steps 3,4 make sure that we take the point modification tiles

    For the placement to be valid, it:
    - Needs to be connected to another word, so either:
        1. The letters needed to complete the word is less than the length of the word or,
        2. There is atleast one perpendicular match
    - Or, it is placed ontop of the "+" symbol (eg the first move)
*/
pub fn execute_move(player: &mut Player, game: &mut Game, play: &PlayWordParams, dict: &ScrabbleDictionary) -> Result<String, String> {
    let board = game.board_to_vec();

    check_parameter_valid(game, play)?;
    check_no_extending_characters(&board, play)?;

    let starting_spot = get_starting_spot(&board);
    
    // Starting spot on the board, play must include it
    if starting_spot.is_some() {
        return play_starting_spot(player, game, play, dict, starting_spot.unwrap());
    }

    Err(String::from("Not implemented yet"))

}

fn play_starting_spot (player: &mut Player, game: &mut Game, play: &PlayWordParams, dict: &ScrabbleDictionary, starting: (usize, usize)) -> Result<String, String> {
    if !dict.is_word_valid(&play.word) {
        return Err(String::from("This is not a valid word"));
    }

    let starting_col = starting.0 as i32;
    let starting_row = starting.1 as i32;


    // Check to make sure the play will include the starting position
    if play.horizontal {
        if (play.row != starting_row || play.col > starting_col || play.col + (play.word.len() as i32) < starting_col) {
            return Err(String::from("The staring point must be included in your play"));
        }
    } else {
        if (play.col != starting_col || play.row > starting_row || play.row + (play.word.len() as i32) < starting_row) {
            return Err(String::from("The staring point must be included in your play"));
        }
    }

    let mut board = game.board_to_vec();

    check_placeable(&board, play, &player.pieces)?;
    let points = get_points_from_place(&board, &play.word, (play.col, play.row), play.horizontal);

    player.points += points;
    place_onto_board(&mut board, play);
    game.set_board_from_vec(board);

    Ok(format!("Player gained {} points", points))
}

/*
    Gets how many points would be given for placing the overlay onto the board.
    Basically if there's a letter on the board, it'll use that to calculate the score.
    Otherwise, it'll calculate the score as if it's placing the character from overlay
    onto the board
*/
fn get_points_from_place(board: &Vec<Vec<char>>, overlay: &str, starting: (i32, i32), horizontal: bool) -> i32 {
    let mut word_multiplier = 1;
    let mut letter_score = 0;

    let (mut col, mut row) = starting;
    let len = overlay.len();

    for i in 0..len {
        let board_char = board[row as usize][col as usize];

        if is_char_letter(board_char) {
            letter_score += get_char_score(board_char);
        } else {
            let (word_mult, letter_mult) = get_multiplier(board_char);
            word_multiplier *= word_mult;

            letter_score += get_char_score(overlay[(i as usize)..].chars().next().unwrap()) * letter_mult;
        }
        if horizontal { col += 1 } else { row += 1 }
    }

    return letter_score * word_multiplier;
}

fn place_onto_board(board: &mut Vec<Vec<char>>, play: &PlayWordParams) {
    let PlayWordParams { mut row, mut col, .. } = play;

    for i in 0..play.word.len() {
        board[row as usize][col as usize] = play.word[(i as usize)..].chars().next().unwrap();
        if play.horizontal { col += 1 } else { row += 1 }
    }
}

fn get_multiplier (c: char) -> (i32, i32) {
    match c {
        '2' => (2, 1),
        '3' => (3, 1),
        '@' => (1, 2),
        '#' => (1, 3),
        _ => (1, 1)
    }
}

fn get_char_score (c: char) -> i32 {
    match c {
        'E' | 'A' | 'I' | 'O' | 'N' | 'R' | 'T' | 'L' | 'S' | 'U' => 1,
        'D' | 'G'  => 2,
        'B' | 'C' | 'M' | 'P' => 3,
        'F' | 'H' | 'V' | 'W' | 'Y' => 4,
        'K' => 5,
        'J' | 'X' => 1,
        'Q' | 'Z' => 1,
        _ => panic!(format!("Trying to get string for invalid char {}", c))
    }
}

fn get_starting_spot(board: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    for (r, row) in board.iter().enumerate() {
        for (c, col) in row.iter().enumerate() {
            if (*col == '+') {
                return Some((r, c));
            }
        }
    }
    return None;
}

fn check_placeable (board: &Vec<Vec<char>>, play: &PlayWordParams, pieces: &str) -> Result<(), String> {
    let mut needed_letters: HashMap<char, i32> = HashMap::new();

    for i in 0..play.word.len() {
        let row: usize = if play.horizontal { play.row as usize } else { play.row as usize + i };
        let col: usize = if play.horizontal { play.col as usize + i } else { play.col as usize };

        let board_letter = board[row][col];
        let word_letter = play.word[i..].chars().next().unwrap();
        if is_char_letter(board_letter) {
            // If the tile on the board is a letter, but the the same one, then there's a collision
            if board_letter != word_letter {
                return Err(String::from("Could not place, word collision"));
            }
        } else {
            *needed_letters.entry(word_letter).or_insert(0) += 1;
        }
    }

    for c in pieces.chars() {
        if needed_letters.contains_key(&c) {
            *needed_letters.entry(c).or_insert(-1) -= 1;
        }
        println!("After {}: {:#?}", c, needed_letters);
    }


    for (key, value) in needed_letters.iter() {
        if *value > 0 {
            return Err(String::from("You do not have enough pieces to do this"));
        }
    }

    Ok(())
}

fn check_parameter_valid(game: &Game, play: &PlayWordParams) -> Result<(), String> {
    if (
        play.col < 0 || play.col >= game.board_width ||
        play.row < 0 || play.row >= game.board_height
    ) {
        return Err(String::from("Row or Col is out of range"));
    }

    for c in play.word.chars() {
        if (c < 'A' || c > 'Z') {
            return Err(String::from("The word can only contain letters A-Z"));
        }
    }

    Ok(())
}

fn check_no_extending_characters(board: &Vec<Vec<char>>, play: &PlayWordParams) -> Result<(), String> {
    if play.horizontal {
        let left = get_char_from_vec(board, play.col - 1, play.row);
        let right = get_char_from_vec(board, play.col + 1, play.row);

        if (left.is_some() && is_char_letter(left.unwrap())) ||
           (right.is_some() && is_char_letter(right.unwrap())) {
               return Err(String::from("There are extending characters, please specify the whole"));
        }    
    } else {
        let up = get_char_from_vec(board, play.col, play.row - 1);
        let down = get_char_from_vec(board, play.col, play.row + 1);

        println!("up: {}, down: {}", up.unwrap(), down.unwrap());

        if (up.is_some() && is_char_letter(up.unwrap())) ||
           (down.is_some() && is_char_letter(down.unwrap())) {
               return Err(String::from("There are extending characters, please specify the whole"));
        }    
    }

    Ok(())
}

fn get_char_from_vec(vec: &Vec<Vec<char>>, x: i32, y: i32) -> Option<char> {
    if y < 0 || y as usize>= vec.len() {
        return None;
    }

    let row = &vec[y as usize];

    if x < 0 || x as usize > row.len() {
        return None;
    } else {
        return Some(row[x as usize]);
    }
}

#[inline]
fn is_char_letter(c: char) -> bool {
    c >= 'A' && c <= 'Z'
}
