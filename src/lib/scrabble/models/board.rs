use super::tile::Tile;

pub struct Board {
    width: usize,
    height: usize,
    tiles: Vec<Tile>
}

impl<'a> From<&'a str> for Board {
    fn from(s: &str) -> Self {
        let tile_count = s.len();
        let dimension = (tile_count as f64).sqrt() as usize;
        let tiles: Vec<Tile> = s.chars().map(|c| Tile::from(c)).collect();

        Board { width: dimension, height: dimension, tiles: tiles }
    }
}

impl Into<String> for Board {
    fn into(self) -> String {
        self.tiles.into_iter().map(|tile| Into::<char>::into(tile)).collect::<String>()
    }
}

#[cfg(test)]
mod tests {
    use super::{Tile, Board};

    #[test]
    fn parses_correct_dimensions() {
        let board_string = ".........";
        let board = Board::from(board_string);

        assert_eq!(board.width, 3);
        assert_eq!(board.height, 3);
    }

    #[test]
    fn parses_correct_dimensions2() {
        let board_string = "....";
        let board = Board::from(board_string);

        assert_eq!(board.width, 2);
        assert_eq!(board.height, 2);
    }

    #[test]
    fn parses_correct_tiles() {
        let board_string = ".23@#+ABC";
        let board = Board::from(board_string);

        assert_eq!(board.tiles[0], Tile::Empty);
        assert_eq!(board.tiles[1], Tile::DoubleWord);
        assert_eq!(board.tiles[2], Tile::TripleWord);
        assert_eq!(board.tiles[3], Tile::DoubleLetter);
        assert_eq!(board.tiles[4], Tile::TripleLetter);
        assert_eq!(board.tiles[5], Tile::Starting);
        assert_eq!(board.tiles[6], Tile::Letter('A'));
        assert_eq!(board.tiles[7], Tile::Letter('B'));
        assert_eq!(board.tiles[8], Tile::Letter('C'));
    }

    #[test]
    fn tile_to_string_identities() {
        let board_string = ".23@#+ABC";
        let board = Board::from(board_string);

        assert_eq!(Into::<String>::into(board), board_string);
    }
}
