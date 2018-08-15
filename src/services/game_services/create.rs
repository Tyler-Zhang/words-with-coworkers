use diesel;
use diesel::PgConnection;
use diesel::prelude::*;
use rand::{thread_rng, Rng};
use ::models::{Game, NewGame};

pub fn create_game(conn: &PgConnection, channel_id: &str, team_id: &str) -> Game {
    use ::schema::*;

    let default_board = generate_default_board();
    let default_pieces = generate_default_pieces();

    let new_game = NewGame {
        board: &default_board,
        turn_count: 0,
        pieces: &default_pieces,
        channel_id: channel_id,
        player_turn_id: None,
        team_id: team_id
    };

    diesel::insert_into(games::table)
        .values(&new_game)
        .get_result(conn)
        .expect("Error saving new game")
}

/*
    This generates the default game board using the following symbols:
    . - Empty piece
    3 - Triple word
    2 - Double word
    @ - Double letter
    # - Tripple letter
    + - Starting spot
*/
fn generate_default_board() -> String {
    "\
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
     3..@...3...@..3".to_string()
}

/*
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pieces_random() {
        assert_ne!(generate_default_pieces(), generate_default_pieces());
    }

    #[test]
    fn pieces_count() {
        assert_eq!(generate_default_pieces().len(), 98);
    }

    #[test]
    fn board_size() {
        assert_eq!(generate_default_board().len(), 15 * 15);
    }
}
