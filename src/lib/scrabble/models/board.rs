use super::{Tile, Direction, Point};
use super::super::config;

pub struct Board {
    pub width: i32,
    pub height: i32,
    pub tiles: Vec<Tile>
}

impl<'a> From<&'a str> for Board {
    fn from(s: &str) -> Self {
        let tile_count = s.len();
        let dimension = (tile_count as f64).sqrt() as i32;
        let tiles: Vec<Tile> = s.chars().map(|c| Tile::from(c)).collect();

        Board { width: dimension, height: dimension, tiles: tiles }
    }
}

impl Into<String> for Board {
    fn into(self) -> String {
        self.tiles.into_iter().map(|tile| Into::<char>::into(tile)).collect::<String>()
    }
}

impl ToString for Board {
    fn to_string(&self) -> String {
        self.tiles.into_iter().map(|tile| Into::<char>::into(tile)).collect::<String>()
    }
}

impl Board {
    pub fn new_default_board() -> Self {
        Board::from(config::DEFAULT_BOARD)
    }

    pub fn check_in_bounds(&self, point: Point) -> bool {
        point.x() < self.width && point.y() < self.height && point.x() >= 0 && point.y() >= 0
    }

    pub fn at(&self, point: Point) -> Result<&Tile, String> {
        if !self.check_in_bounds(point) {
            return Err(format!("Coordinates out of bounds"));
        }

        Ok(&self.tiles[(point.y() * self.width + point.x()) as usize])
    }

    pub fn get_starting_spot(&self) -> Option<Point> {
        let mut index: i32 = 0;
        for tile in self.tiles.iter() {
            if *tile == Tile::Starting {
                break;
            }
            index += 1;
        }

        if index < self.tiles.len() as i32 {
            return Some(Point::new(index % self.width, index / self.width));
        }

        None
    }

    pub fn iterate_for<F>(&self, start: Point, direction: Direction, len: i32, mut f: F)
    where
        F: FnMut(&Tile) -> ()
    {
        if len <= 0 {
            return;
        }

        assert_eq!(self.check_in_bounds(start + direction * (len - 1)), true);

        let mut curr = start;

        for i in 0..len {
            f(self.at(curr).unwrap());

            curr += direction;
        }
    }

    pub fn iterate_while<F>(&self, start: Point, direction: Direction, mut f: F)
    where
        F: FnMut(&Tile) -> bool
    {
        let mut curr = start;

        loop {
            if !self.check_in_bounds(curr) || !f(self.at(curr).unwrap()){
                return;
            }
            curr += direction;
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iterate_while() {
        let board = get_board();
        let expected_tiles = vec!(Tile::Empty, Tile::TripleLetter, Tile::Letter('C'));

        let mut idx = 0;

        board.iterate_while(Point::new(0, 0), Direction::new(1, 1), |tile: &Tile| -> bool {
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
    fn test_iterate_for() {
        let board = get_board();
        let expected_tiles = vec!(Tile::Empty, Tile::TripleLetter, Tile::Letter('C'));

        let mut idx = 0;

        board.iterate_for(Point::new(0, 0), Direction::new(1, 1), 3, |tile: &Tile| {
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

        assert_eq!(board.check_in_bounds(Point::new(0, 0)), true);
        assert_eq!(board.check_in_bounds(Point::new(2, 2)), true);
        assert_eq!(board.check_in_bounds(Point::new(3, 3)), false);
    }

    #[test]
    fn test_at() {
        let board = get_board();

        assert_eq!(board.at(Point::new(0, 0)).is_ok(), true);
        assert_eq!(board.at(Point::new(0, 0)).unwrap(), &Tile::Empty);

        assert_eq!(board.at(Point::new(1, 2)).is_ok(), true);
        assert_eq!(board.at(Point::new(1, 2)).unwrap(), &Tile::Letter('B'));

        assert_eq!(board.at(Point::new(3, 0)).is_err(), true);
    }

    #[test]
    fn get_starting_spot() {
        let board = get_board();
        assert_eq!(board.get_starting_spot(), Some(Point::new(2, 1)));

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
