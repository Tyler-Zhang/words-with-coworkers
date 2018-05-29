use ::models::{Player, Game};
use ::app::slack::play::PlayWordParams;
use app::state::dictionary::ScrabbleDictionary;

mod checkers;
mod helpers;
use self::helpers::*;
use self::checkers::*;

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

    return play_regular(player, game, play, dict);
}

fn play_regular(player: &mut Player, game: &mut Game, play: &PlayWordParams, dict: &ScrabbleDictionary) -> Result<String, String> {
    check_valid_word(&play.word, dict)?;
    
    let mut board = game.board_to_vec();

    check_placeable(&board, play, &player.pieces)?;

    let mut points = get_points_from_place(&board, &play.word, (play.col, play.row), play.horizontal);

    // Go along the word, and use extend word to calculate all perpendicular word values
    let mut x = play.col;
    let mut y = play.row;

    let xDelta = if play.horizontal { 1 } else { 0 };
    let yDelta = if play.horizontal { 0 } else { 1 };

    let mut did_find_perpendicular = false;

    for i in 0..play.word.len() {
        let (extended_word, starts_at) = extend_word_both_dir(&board, (x, y), !play.horizontal, play.word[i..].chars().next().unwrap());

        // Dont check if it's just one letter
        if extended_word.len() > 1 {
            check_valid_word(&extended_word, dict)?;
            points += get_points_from_place(&board, &extended_word, starts_at, !play.horizontal);
            did_find_perpendicular = true;
        }

        x += xDelta;
        y += yDelta;
    }

    // Everything was successful, give the player their points
    player.points += points;

    let used_pieces = place_onto_board(&mut board, play);

    if used_pieces.len() == play.word.len() && !did_find_perpendicular {
        return Err(format!("Your play must intersect with another word"));
    }


    super::pieces::remove_pieces(&mut *player, &used_pieces)?;
    super::pieces::give_pieces(&mut *player, &mut *game);
    game.set_board_from_vec(board);

    Ok(format!("Player gained {} points", points))
}


fn play_starting_spot (player: &mut Player, game: &mut Game, play: &PlayWordParams, dict: &ScrabbleDictionary, starting: (usize, usize)) -> Result<String, String> {
    check_valid_word(&play.word, dict)?;

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
 
    let used_pieces = place_onto_board(&mut board, play);
    super::pieces::remove_pieces(&mut *player, &used_pieces)?;
    super::pieces::give_pieces(&mut *player, &mut *game);
    game.set_board_from_vec(board);

    // Remove pieces from player

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

fn place_onto_board(board: &mut Vec<Vec<char>>, play: &PlayWordParams) -> String {
    let PlayWordParams { mut row, mut col, .. } = play;

    let mut used_letters = String::new();

    for i in 0..play.word.len() {
        if !is_char_letter(board[row as usize][col as usize]) {
            let word_letter = play.word[(i as usize)..].chars().next().unwrap();
            board[row as usize][col as usize] = word_letter;
            used_letters.push(word_letter);
        }
        if play.horizontal { col += 1 } else { row += 1 }
    }

    used_letters
}
