use rand::{thread_rng, Rng};

pub static PLAYER_HAND_PIECES_COUNT: i32 = 7;

/*
    This generates the default game board using the following symbols:
    . - Empty piece
    3 - Triple word
    2 - Double word
    @ - Double letter
    # - Tripple letter
    + - Starting spot
*/
pub static DEFAULT_BOARD: &str ="\
    3..@...3...@..3\
    .2...#...#...2.\
    ..2...@.@...2..\
    @..2...@...2..@\
    ....2.....2....\
    .#...#...#...#.\
    ..@...@.@...@..\
    3..@...+...@..3\
    ..@...@.@...@..\
    .#...#...#...#.\
    ....2.....2....\
    @..2...@...2..@\
    ..2...@.@...2..\
    .2...#...#...2.\
    3..@...3...@..3";

pub static BINGO_LETTERS_PLAYED: i32 = 7;

pub static BINGO_POINT_VALUE: i32 = 70;

/*as
    This generates the inital "bag" of pieces that have not been picked
    We currently dont support the blank tile
*/
pub fn generate_default_pieces() -> String {
    let pieces = [
        "E".repeat(12),
        "A".repeat(9),
        "I".repeat(9),
        "O".repeat(8),
        "N".repeat(6),
        "R".repeat(6),
        "T".repeat(6),
        "L".repeat(4),
        "S".repeat(4),
        "U".repeat(4),

        "D".repeat(4),
        "G".repeat(3),

        "B".repeat(2),
        "C".repeat(2),
        "M".repeat(2),
        "P".repeat(2),

        "F".repeat(2),
        "H".repeat(2),
        "V".repeat(2),
        "W".repeat(2),
        "Y".repeat(2),

        "K".repeat(1),

        "J".repeat(1),
        "X".repeat(1),

        "Q".repeat(1),
        "Z".repeat(1)
    ].join("");

    let mut shuffled: Vec<u8> = pieces.into_bytes();
    thread_rng().shuffle(&mut shuffled);

    String::from_utf8(shuffled).expect("Shuffle pieces")
}

// Get how much a letter is worth
pub fn get_letter_score (c: char) -> i32 {
    match c {
        'E' | 'A' | 'I' | 'O' | 'N' | 'R' | 'T' | 'L' | 'S' | 'U' => 1,
        'D' | 'G'  => 2,
        'B' | 'C' | 'M' | 'P' => 3,
        'F' | 'H' | 'V' | 'W' | 'Y' => 4,
        'K' => 5,
        'J' | 'X' => 8,
        'Q' | 'Z' => 10,
        _ => panic!(format!("Trying to get string for invalid char {}", c))
    }
}
