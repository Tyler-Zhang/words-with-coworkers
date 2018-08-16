use super::super::config;
use super::board::Board;
use super::player::Player;
use rand::{thread_rng, Rng};

pub struct Game {
    board: Board,
    players: Vec<Player>,
    player_turn: u32,
    pieces: String
}

impl Game {
    fn generate_new_game(player_count: u32) -> Self {
        let pieces = generate_default_pieces();

        let players = [0..player_count].iter()
            .map(|_| Player::new(pieces.split_off(pieces.len() - config::PLAYER_HAND_PIECES_COUNT as usize)))
            .collect();

        Game {
            board: Board::new_default_board(),
            players: players,
            player_turn: 0,
            pieces: pieces
        }
    }
}

/*as
    This generates the inital "bag" of pieces that have not been picked
    We currently dont support the blank tile
*/
fn generate_default_pieces() -> String {
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
