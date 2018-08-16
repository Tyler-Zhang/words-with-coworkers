use super::tile::Tile;

pub struct Board {
    width: u32,
    height: u32,
    tiles: Vec<Tile>
}

impl<'a> From<&'a str> for Board {
    fn from(s: &str) -> Self {
        let tile_count = s.len();
        let dimension = (tile_count as f64).sqrt() as u32;
        let tiles: Vec<Tile> = s.chars().map(|c| Tile::from(c)).collect();

        Board { width: dimension, height: dimension, tiles: tiles }
    }
}

impl Into<String> for Board {
    fn into(self) -> String {
        self.tiles.into_iter().map(|tile| Into::<char>::into(tile)).collect::<String>()
    }
}

impl Board {
    pub fn new_default_board() -> Self {
        /*
            This generates the default game board using the following symbols:
            . - Empty piece
            3 - Triple word
            2 - Double word
            @ - Double letter
            # - Tripple letter
            + - Starting spot
        */
        return Board::from(
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
            3..@...3...@..3"
        )
    }

    fn in_bounds(&self, x: u32, y: u32) -> bool {
        x < self.width && y < self.height
    }

    fn at(&self, x: u32, y: u32) -> Result<&Tile, String> {
        if !self.in_bounds(x, y) {
            return Err(format!("Coordinates out of bounds"));
        }

        Ok(&self.tiles[(y * self.width + x) as usize])
    }

    fn get_starting_spot(&self) -> Option<(u32, u32)> {
        let mut index: u32 = 0;
        for tile in self.tiles.iter() {
            if *tile == Tile::Starting {
                break;
            }
            index += 1;
        }

        if index < self.tiles.len() as u32 {
            return Some((index % self.width, index / self.width));
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::{Tile, Board};

    #[test]
    fn parses_correct_dimensions2() {
        let board_string = "....";
        let board = Board::from(board_string);

        assert_eq!(board.width, 2);
        assert_eq!(board.height, 2);
    }

    #[test]
    fn parses_correctly() {
        let board_string = ".23@#+ABC";
        let board = Board::from(board_string);

        assert_eq!(board.width, 3);
        assert_eq!(board.height, 3);
        assert_eq!(board.tiles[0], Tile::Empty);
        assert_eq!(board.tiles[1], Tile::DoubleWord);
        assert_eq!(board.tiles[2], Tile::TripleWord);
    }

    #[test]
    fn tile_to_string_identities() {
        let board_string = ".23@#+ABC";
        let board = Board::from(board_string);

        assert_eq!(Into::<String>::into(board), board_string);
    }

    #[test]
    fn in_bounds() {
        let board_string = ".23@#+ABC";
        let board = Board::from(board_string);

        assert_eq!(board.in_bounds(0, 0), true);
        assert_eq!(board.in_bounds(2, 2), true);
        assert_eq!(board.in_bounds(3, 3), false);
    }

    #[test]
    fn at() {
        let board_string = ".23@#+ABC";
        let board = Board::from(board_string);

        assert_eq!(board.at(0, 0).is_ok(), true);
        assert_eq!(board.at(0, 0).unwrap(), &Tile::Empty);

        assert_eq!(board.at(1, 2).is_ok(), true);
        assert_eq!(board.at(1, 2).unwrap(), &Tile::Letter('B'));

        assert_eq!(board.at(3, 0).is_err(), true);
    }

    #[test]
    fn get_starting_spot() {
        let board_string = ".23@#+ABC";
        let board = Board::from(board_string);
        assert_eq!(board.get_starting_spot(), Some((2, 1)));

        let board_string = ".23@#.ABC";
        let board = Board::from(board_string);
        assert_eq!(board.get_starting_spot(), None);
    }

    #[test]
    fn new_default_board() {
        let board = Board::new_default_board();

        assert_eq!(board.width, 15);
        assert_eq!(board.height, 15);
    }
}
