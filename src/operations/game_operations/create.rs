use diesel;
use diesel::PgConnection;
use diesel::prelude::*;
use ::models::{Game, NewGame};

pub fn create_game<'a>(conn: &PgConnection, channel_id: &'a str) -> Game {
    use ::schema::*;

    let new_game = NewGame {
        board: generate_default_board(),
        board_width: 15,
        board_height: 15,
        turn_count: 0,
        pieces: generate_default_pieces(),
        channel_id: channel_id,
        player_turn_id: None
    };

    diesel::insert_into(games::table)
        .values(&new_game)
        .get_result(conn)
        .expect("Error saving new game")
}

fn generate_default_board<'a>() -> &'a str {
    "\
    3  @   3   @  3\
     2   #   #   2 \
      2   @ @   2  \
    @  2   @   2  2\
        2     2    \
     #   #   #   # \
    3  @   .   @  3\
     #   #   #   # \
        2     2    \
    @  2   @   2  2\
      2   @ @   2  \
     2   #   #   2 \
    3  @   3   @  3"
}

fn generate_default_pieces<'a>() -> &'a str {
    "EEEEEEEEEEEEAAAAAAAAAIIIIIIIIIOOOOOOOONNNNNNRRRRRRTTTTTTLLLLSSSSUUUU"
}
