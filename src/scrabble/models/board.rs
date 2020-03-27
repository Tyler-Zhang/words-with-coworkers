use super::super::constants::{BOARD, BOARD_SIZE};
use super::super::error::{Error, Result};
use super::tile::Tile;
use super::{Direction, Point, Strip};

/**
 * Represents how the cell on the board affects the scoring of the final word
 */
#[derive(Debug, PartialEq)]
pub struct BoardCellMultiplier {
    word: u32,
    letter: u32,
}

impl BoardCellMultiplier {
    pub fn new(word: u32, letter: u32) -> Self {
        BoardCellMultiplier {
            word: word,
            letter: letter,
        }
    }
}

/**
 * Represents the state of a cell on the board
 */
#[derive(Debug, PartialEq)]
pub enum BoardCell {
    StartingSpot,
    Empty,
    DoubleLetter,
    TripleLetter,
    DoubleWord,
    TripleWord,
    Tile(Tile),
}

impl BoardCell {
    pub fn get_multiplier(&self) -> BoardCellMultiplier {
        match self {
            Self::DoubleLetter => BoardCellMultiplier::new(1, 2),
            Self::TripleLetter => BoardCellMultiplier::new(1, 3),
            Self::DoubleWord => BoardCellMultiplier::new(2, 1),
            Self::TripleWord => BoardCellMultiplier::new(3, 1),
            _ => BoardCellMultiplier::new(1, 1),
        }
    }
}

/**
 * Trait defines basic operations that can be performed on a board
 */
trait ReadableBoard {
    fn is_in_bounds(&self, x: u32, y: u32) -> bool;
    fn get(&self, x: u32, y: u32) -> Option<&BoardCell>;
}

#[inline]
fn xy_to_idx(width: u32, x: u32, y: u32) -> usize {
    (y * width + x) as usize
}

#[derive(Debug)]
pub struct Board {
    pub cells: Vec<BoardCell>,
}

impl ReadableBoard for Board {
    #[inline]
    fn is_in_bounds(&self, x: u32, y: u32) -> bool {
        (0..BOARD_SIZE).contains(&x) && (0..BOARD_SIZE).contains(&y)
    }

    fn get(&self, x: u32, y: u32) -> Option<&BoardCell> {
        if !self.is_in_bounds(x, y) {
            return None;
        }

        self.cells.get(xy_to_idx(BOARD_SIZE, x, y))
    }
}

impl Board {
    pub fn new() -> Board {
        let mut cells = Vec::with_capacity((BOARD_SIZE * BOARD_SIZE) as usize);

        for c in BOARD.chars() {
            cells.push(match c {
                '.' => BoardCell::Empty,
                '3' => BoardCell::TripleWord,
                '2' => BoardCell::DoubleWord,
                '@' => BoardCell::DoubleLetter,
                '#' => BoardCell::TripleLetter,
                '+' => BoardCell::StartingSpot,
                _ => unreachable!("Tried to parse invalid character for board"),
            })
        }

        Board { cells }
    }

    fn set(&mut self, x: u32, y: u32, bc: BoardCell) -> Result<()> {
        if !self.is_in_bounds(x, y) {
            return Err(Error::BadAction(format!("Out of bounds")).into());
        }
        self.cells[xy_to_idx(BOARD_SIZE, x, y)] = bc;
        Ok(())
    }
}

/**
 * A decorator for a Board that places an "uncommitted" line of pieces above
 * a line on the original board
 */
pub struct BoardWithOverlay<'a> {
    board: Board,
    strip: Strip,
    word: &'a str,
    board_cells: Vec<Option<BoardCell>>,
}

impl<'a> BoardWithOverlay<'a> {
    fn get_overlay_mask(board: &Board, strip: &Strip, word: &str) -> Result<Vec<Option<BoardCell>>> {
        let mut curr_point = strip.start.clone();

        let mut mask = Vec::<Option<BoardCell>>::with_capacity(strip.len as usize);

        for curr_letter in word.chars() {
            let board_cell = board.get(curr_point.x as u32, curr_point.y as u32).ok_or(
                Error::BadAction(format!("This placement goes off of the board!"))
            )?;

            match board_cell {
                BoardCell::Tile(Tile::Letter(letter)) =>
                    if letter == &curr_letter {
                        mask.push(None);
                    } else {
                        return Err(Error::BadAction(format!("Pieces do not fit")).into());
                    },
                _ => mask.push(Some(BoardCell::Tile(Tile::Letter(curr_letter))))
            }

            curr_point += &strip.dir;
        }

        Ok(mask)
    }

    pub fn try_overlay<'b>(
        board: Board,
        point: Point,
        dir: Direction,
        word: &'b str,
    ) -> Result<BoardWithOverlay<'b>> {
        let strip = Strip::new(point, dir, word.len() as i32);

        let overlay_mask = Self::get_overlay_mask(&board, &strip, word)?;

        let bwo = BoardWithOverlay {
            board,
            strip,
            word,
            board_cells: overlay_mask
        };

        Ok(bwo)
    }

    pub fn get_overlaid_letters(&self) -> Vec<Tile> {
        self.board_cells.iter()
            .filter(|w| w.is_some())
            .map(|w| {
                match w {
                    &Some(BoardCell::Tile(tile)) => tile,
                    _ => unreachable!()
                }
            })
            .collect()
    }
}
impl<'a> ReadableBoard for BoardWithOverlay<'a> {
    fn is_in_bounds(&self, x: u32, y: u32) -> bool {
        self.board.is_in_bounds(x, y)
    }

    fn get(&self, x: u32, y: u32) -> Option<&BoardCell> {
        if !self.is_in_bounds(x, y) {
            return None;
        }

        if !self.strip.contains(&Point::new(x as i32, y as i32)) {
            // This piece is not being overlayed on
            return self.board.get(x, y);
        }

        // Get distance from required piece to the start of the strip
        let distance = (x as i32 - self.strip.start.x as i32).abs() +
                       (y as i32 - self.strip.start.y as i32).abs();

        // If we are looking at a cell that is actually being overlayed,
        // and not just a part of the strip, then we return the cell
        // otherwise, we return the underlying piece
        if let Some(ref cell) = self.board_cells[distance as usize] {
            return Some(cell);
        } else {
            return self.board.get(x, y);
        }
    }
}

impl dyn ReadableBoard {
    /**
     * Allows us to easily iterate over a line of the board
     *
     * The iterator function can control if it want's to continue iterator
     * by returning a result. An Err will immediately end the iteration
     */
    pub fn for_each(
        &self,
        strip: &Strip,
        f: &dyn Fn(&Point, &BoardCell) -> bool,
    ) {
        let mut loc = (strip.start).clone();

        for _ in 0..strip.len {
            if let Some(bc) = self.get(loc.x as u32, loc.y as u32) {
                if !f(&loc, bc) {
                    return;
                }
            } else {
                return;
            }

            loc += &strip.dir;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_no_tile_test() {
        let board = Board::new();

        assert_eq!(board.get(0, 0).unwrap(), &BoardCell::TripleWord);
        assert_eq!(board.get(3, 0).unwrap(), &BoardCell::DoubleLetter);
        assert_eq!(board.get(1, 1).unwrap(), &BoardCell::DoubleWord);
        assert_eq!(board.get(7, 7).unwrap(), &BoardCell::StartingSpot);
        assert_eq!(board.get(BOARD_SIZE, BOARD_SIZE), None);
    }

    #[test]
    fn set_and_get_tiles() {
        let mut board = Board::new();

        assert_eq!(
            board.set(0, 0, BoardCell::Tile(Tile::Letter('A'))).is_ok(),
            true
        );
        assert_eq!(board.get(0, 0).unwrap(), &BoardCell::Tile(Tile::Letter('A')));

        assert_eq!(
            board.set(BOARD_SIZE, BOARD_SIZE, BoardCell::Empty).is_err(),
            true
        );
    }

    #[test]
    fn pieces_for_place() {
        let mut board = Board::new();
        let mut board_with_overlay = BoardWithOverlay::try_overlay(
            board,
            Point::new(0,0),
            Direction::new(1, 0),
            "HI"
        ).unwrap();

        assert_eq!(
            board_with_overlay.get_overlaid_letters(),
            vec![Tile::Letter('H'), Tile::Letter('I')]
        );

        board = Board::new();
        board.set(1, 0, BoardCell::Tile(Tile::Letter('E'))).unwrap();
        board.set(3, 0, BoardCell::Tile(Tile::Letter('L'))).unwrap();
        board_with_overlay = BoardWithOverlay::try_overlay(
            board,
            Point::new(0, 0),
            Direction::new(1, 0),
            "HELLO"
        ).unwrap();

        assert_eq!(
            board_with_overlay.get_overlaid_letters(),
            vec![Tile::Letter('H'), Tile::Letter('L'), Tile::Letter('O')]
        );
    }

    #[test]
    fn pieces_for_place_err() {
        let mut board = Board::new();
        let mut board_with_overlay = BoardWithOverlay::try_overlay(
            board,
            Point::new(0, 0),
            Direction::new(1, 0),
            "REALLY LONG WORD THAT OVERFLOWS THE ENTIRE BOARD"
        );

        assert_eq!(board_with_overlay.is_err(), true);

        board = Board::new();
        board_with_overlay = BoardWithOverlay::try_overlay(
            board,
            Point::new(10, 0),
            Direction::new(1, 0),
            "LONGWORD"
        );

        assert_eq!(board_with_overlay.is_err(), true);
    }
}
