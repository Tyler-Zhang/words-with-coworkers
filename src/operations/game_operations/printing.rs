use ::models::GameAdapter;
use ::models::Player;
use ::lib::{scrabble, slack};

use self::scrabble::Tile;
use self::slack::emoji::{Emoji, ScrabbleBoardTile};
use self::slack::tag::Tag;

impl scrabble::Tile {
    pub fn to_emoji(&self) -> Emoji {
        match self {
            &Tile::DoubleLetter => Emoji::ScrabbleBoardTile(ScrabbleBoardTile::DoubleLetter),
            &Tile::TripleLetter => Emoji::ScrabbleBoardTile(ScrabbleBoardTile::TripleLetter),
            &Tile::DoubleWord => Emoji::ScrabbleBoardTile(ScrabbleBoardTile::DoubleWord),
            &Tile::TripleWord => Emoji::ScrabbleBoardTile(ScrabbleBoardTile::TripleWord),
            &Tile::Empty => Emoji::ScrabbleBoardTile(ScrabbleBoardTile::Board),
            &Tile::Starting => Emoji::ScrabbleBoardTile(ScrabbleBoardTile::Start),
            &Tile::Letter(c) => Emoji::ScrabbleLetter(c)
        }
    }
}

pub fn format_game(game: &GameAdapter, include_players: bool) -> String {
    format!("\
        It is currently {}'s turn
        \n{}\
        \n{}\
        ",
        Tag(&game.get_player_on_turn().slack_id).to_string(),
        board_to_string(&game.scrabble_game.board),
        if include_players { players_to_str(&game.db_players) } else { "".to_string() }
    )
}

fn players_to_str(players: &Vec<Player>) -> String {
    players.iter().map(|player| format!("{} - {}", Tag(&player.slack_id).to_string(), player.points))
        .collect::<Vec<String>>()
        .join("\n")

}

fn board_to_string(board: &scrabble::Board) -> String {
    let width = board.width;

    let mut printout = String::new();

    // Add headers in
    printout += &Emoji::WhiteSquare.to_string();

    for i in 0..board.width {
        printout += &Emoji::Number(i % 10).to_string();
    }
    printout += "\n";

    // Enumerate through tiles
    for (index, tile) in board.tiles.iter().enumerate() {
        let index = index as i32;

        if index % board.width == 0 {
            if index != 0 {
                printout += "\n";
            }

            printout += &Emoji::Number(index/board.width % 10).to_string();
        }

        printout += &tile.to_emoji().to_string();
    }

    printout
}
