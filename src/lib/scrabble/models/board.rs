use super::{Tile, Action};
use super::super::config;

pub struct Board {
    pub width: u32,
    pub height: u32,
    pub tiles: Vec<Tile>
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
        Board::from(config::DEFAULT_BOARD)
    }

    pub fn check_in_bounds(&self, x: u32, y: u32) -> bool {
        x < self.width && y < self.height
    }

    pub fn at(&self, x: u32, y: u32) -> Result<&Tile, String> {
        if !self.check_in_bounds(x, y) {
            return Err(format!("Coordinates out of bounds"));
        }

        Ok(&self.tiles[(y * self.width + x) as usize])
    }

    pub fn get_starting_spot(&self) -> Option<(u32, u32)> {
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

    pub fn iterate<F>(&self, start: (u32, u32), direction: (u32, u32), len: u32, mut f: F)
    where
        F: FnMut(&Tile) -> ()
    {
        if len == 0 {
            return;
        }

        let end_x = start.0 + direction.0 * (len - 1);
        let end_y = start.1 + direction.1 * (len - 1);

        assert_eq!(self.check_in_bounds(end_x, end_y), true);

        let mut x = start.0;
        let mut y = start.1;

        for i in 0..len {
            f(self.at(x, y).unwrap());

            x += direction.0;
            y += direction.1;
        }
    }

    pub fn iterate_until<F>(&self, start: (u32, u32), direction: (u32, u32), mut f: F)
    where
        F: FnMut(&Tile) -> bool
    {
        let mut x = start.0;
        let mut y = start.1;

        loop {
            if !self.check_in_bounds(x, y) || !f(self.at(x, y).unwrap()){
                return;
            }

            x += direction.0;
            y += direction.1;
        }
    }

    pub fn extend_word(&self, action: &mut Action) {
    }
}


#[cfg(test)]
mod tests {
    use super::{Tile, Board};

    #[test]
    fn test_iterate_until() {
        let board = get_board();
        let expected_tiles = vec!(Tile::Empty, Tile::TripleLetter, Tile::Letter('C'));

        let mut idx = 0;

        board.iterate_until((0, 0), (1, 1), |tile: &Tile| -> bool {
            if idx == 2 {
                return false;
            }

            assert_eq!(tile, &expected_tiles[idx]);
            idx += 1;
            return true;
        });

        assert_eq!(idx, 2);
    }


    #[test]
    fn test_iterate() {
        let board = get_board();
        let expected_tiles = vec!(Tile::Empty, Tile::TripleLetter, Tile::Letter('C'));

        let mut idx = 0;

        board.iterate((0, 0), (1, 1), 3, |tile: &Tile| {
            assert_eq!(tile, &expected_tiles[idx]);
            idx += 1;
        });

        assert_eq!(idx, 3);
    }

    #[test]
    fn parses_correct_dimensions2() {
        let board_string = "....";
        let board = Board::from(board_string);

        assert_eq!(board.width, 2);
        assert_eq!(board.height, 2);
    }

    #[test]
    fn parses_correctly() {
        let board = get_board();

        assert_eq!(board.width, 3);
        assert_eq!(board.height, 3);
        assert_eq!(board.tiles[0], Tile::Empty);
        assert_eq!(board.tiles[1], Tile::DoubleWord);
        assert_eq!(board.tiles[2], Tile::TripleWord);
    }

    #[test]
    fn tile_to_string_identities() {
        let board = get_board();

        assert_eq!(Into::<String>::into(board), ".23@#+ABC");
    }

    #[test]
    fn check_in_bounds() {
        let board = get_board();

        assert_eq!(board.check_in_bounds(0, 0), true);
        assert_eq!(board.check_in_bounds(2, 2), true);
        assert_eq!(board.check_in_bounds(3, 3), false);
    }

    #[test]
    fn test_at() {
        let board = get_board();

        assert_eq!(board.at(0, 0).is_ok(), true);
        assert_eq!(board.at(0, 0).unwrap(), &Tile::Empty);

        assert_eq!(board.at(1, 2).is_ok(), true);
        assert_eq!(board.at(1, 2).unwrap(), &Tile::Letter('B'));

        assert_eq!(board.at(3, 0).is_err(), true);
    }

    #[test]
    fn get_starting_spot() {
        let board = get_board();
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

    fn get_board() -> Board {
        let board_string = "\
        .23\
        @#+\
        ABC";
        return Board::from(board_string);
    }
}
