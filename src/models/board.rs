use std::fmt;
use super::super::constants::{BOARD, BOARD_SIZE};
use super::super::error::{Error, Result};
use super::tile::Tile;
use super::{Direction, Point, Strip};

/**
 * Represents how the cell on the board affects the scoring of the final word
 */
#[derive(Debug, PartialEq)]
pub struct BoardCellMultiplier {
    pub word: u32,
    pub letter: u32,
}

impl BoardCellMultiplier {
    pub fn new(word: u32, letter: u32) -> Self {
        BoardCellMultiplier {
            word,
            letter,
        }
    }
}

/**
 * Represents the state of a cell on the board
 */
#[derive(Debug, PartialEq, Clone)]
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

    pub fn to_letter(&self) -> char {
        match self {
            Self::StartingSpot => '+',
            Self::Empty => '.',
            Self::DoubleLetter => '@',
            Self::TripleLetter => '#',
            Self::DoubleWord => '2',
            Self::TripleWord => '3',
            Self::Tile(Tile::Letter(letter)) => *letter,
            _ => unreachable!()
        }
    }
}

impl fmt::Display for BoardCell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_letter())
    }
}

/**
 * Trait defines basic operations that can be performed on a board
 */
trait ReadableBoard {
    fn is_in_bounds(&self, point: &Point) -> bool;
    fn get(&self, point: &Point) -> Option<&BoardCell>;
}


#[inline]
fn xy_to_idx(width: u32, point: &Point) -> usize {
    (point.y * width as i32 + point.x) as usize
}

#[derive(Debug, Clone)]
pub struct Board {
    pub cells: Vec<BoardCell>,
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..BOARD_SIZE {
            for x in 0..BOARD_SIZE {
                self.get(&Point::new(x as i32, y as i32)).unwrap().fmt(f)?;
            }
            writeln!(f)?
        }
        Ok(())
    }
}

impl ReadableBoard for Board {
    #[inline]
    fn is_in_bounds(&self, point: &Point) -> bool {
        (0..BOARD_SIZE).contains(&(point.x as u32)) && (0..BOARD_SIZE).contains(&(point.y as u32))
    }

    fn get(&self, point: &Point) -> Option<&BoardCell> {
        if !self.is_in_bounds(point) {
            return None;
        }

        self.cells.get(xy_to_idx(BOARD_SIZE, point))
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

    #[allow(dead_code)]
    fn set(&mut self, point: &Point, bc: BoardCell) -> Result<()> {
        if !self.is_in_bounds(point) {
            return Err(Error::BadAction(format!("Out of bounds")).into());
        }
        self.cells[xy_to_idx(BOARD_SIZE, point)] = bc;
        Ok(())
    }

    fn get_mut(&mut self, point: &Point) -> Option<&mut BoardCell> {
        if !self.is_in_bounds(point) {
            return None;
        }

        self.cells.get_mut(xy_to_idx(BOARD_SIZE, point))
    }

    fn for_each_mut(
        &mut self,
        strip: &Strip,
        f: &mut dyn FnMut(&Point, &mut BoardCell) -> bool,
    ) {
        let mut loc = (strip.start).clone();

        for _ in 0..strip.len {
            if let Some(bc) = self.get_mut(&loc) {
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

/**
 * A decorator for a Board that places an "uncommitted" line of pieces above
 * a line on the original board
 */
pub struct BoardWithOverlay {
    board: Board,
    strip: Strip,
    board_cells: Vec<Option<BoardCell>>,
}

pub type OverlaidWord = Vec<(BoardCell, Option<BoardCell>)>;

impl BoardWithOverlay {
    fn get_overlay_mask(board: &Board, strip: &Strip, word: &str) -> Result<Vec<Option<BoardCell>>> {
        let mut curr_point = strip.start.clone();

        let mut mask = Vec::<Option<BoardCell>>::with_capacity(strip.len as usize);

        for curr_letter in word.chars() {
            let board_cell = board.get(&curr_point).ok_or(
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

    pub fn try_overlay(
        board: Board,
        point: Point,
        dir: Direction,
        word: &str,
    ) -> Result<BoardWithOverlay> {
        let strip = Strip::new(point, dir, word.len() as i32);

        let overlay_mask = Self::get_overlay_mask(&board, &strip, word)?;

        let bwo = BoardWithOverlay {
            board,
            strip,
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

    fn get_overlay_at(&self, point: &Point) -> Option<&BoardCell> {
        let dist_from_start = self.strip.distance_in(&point)?;

        self.board_cells[dist_from_start as usize].as_ref()
    }

    fn is_point_covered(&self, point: &Point) -> bool {
        self.get_overlay_at(point).is_some()
    }

    /// Returns a vector tuple. The first element stores the flattened tile
    /// (i.e which ever tile .get returns) where as the second element
    /// stores the tile underneath if the first is an overlaid tile
    pub fn get_connecting_letters_from(&self, start: Point, dir: Direction) -> Vec<(BoardCell, Option<BoardCell>)> {
        let mut accum_vec = Vec::<(BoardCell, Option<BoardCell>)>::new();

        self.for_each_until(
            start,
            dir,
            &mut |point, board_cell| {
                match board_cell {
                    &BoardCell::Tile(_) => {
                        accum_vec.push((
                            (*board_cell).clone(),
                            if self.strip.contains(point) {
                                Some((*self.board.get(point).unwrap()).clone())
                            } else {
                                None
                            })
                        );
                        true
                    }
                    _ => false
                }
            }
        );

        accum_vec
    }

    pub fn get_whole_word(&self, start: &Point, dir: &Direction) -> OverlaidWord {
        let start = *start;
        let dir = *dir;

        let mut word = Vec::new();

        let mut opposite_dir = self.get_connecting_letters_from(start + (dir * -1), dir * -1);
        opposite_dir.reverse();

        word.append(&mut opposite_dir);
        word.append(&mut self.get_connecting_letters_from(start, dir));

        word
    }

    pub fn get_formed_words(&self) -> (OverlaidWord, Vec<OverlaidWord>){
        let main_line_word = self.get_whole_word(&self.strip.start, &self.strip.dir);

        let mut branching_words = Vec::new();

        let perp_direction = if self.strip.dir.is_horizontal() {
            Direction::down()
        } else {
            Direction::right()
        };

        self.for_each(
            &self.strip,
            &mut |point, _| {
                if self.is_point_covered(point) {
                    let word = self.get_whole_word(point, &perp_direction);

                    if word.len() > 1 {
                        branching_words.push(word)
                    }
                }
                true
            }
        );

        (main_line_word, branching_words)
    }

    pub fn apply_to_board(mut self) -> Board {
        let mut i = 0usize;
        let board_cells = &self.board_cells;

        self.board.for_each_mut(&self.strip, &mut |_, board_cell| {
            if let Some(ref new_board_cell) = board_cells[i] {
                *board_cell = new_board_cell.clone();
            }
            i +=1;
            true
        });

        self.board
    }
}

impl ReadableBoard for BoardWithOverlay {
    fn is_in_bounds(&self, point: &Point) -> bool {
        self.board.is_in_bounds(point)
    }

    fn get(&self, point: &Point) -> Option<&BoardCell> {
        if !self.is_in_bounds(point) {
            return None;
        }

        if !self.strip.contains(&point) {
            // This piece is not being overlayed on
            return self.board.get(point);
        }

        // If we are looking at a cell that is actually being overlayed,
        // and not just a part of the strip, then we return the cell
        // otherwise, we return the underlying piece
        if let Some(ref cell) = self.get_overlay_at(point) {
            Some(cell)
        } else {
            self.board.get(point)
        }
    }
}

trait IterableBoard {
    fn for_each_until(
        &self,
        start: Point,
        dir: Direction,
        f: &mut dyn FnMut(&Point, &BoardCell) -> bool,
    );

    fn for_each(
        &self,
        strip: &Strip,
        f: &mut dyn FnMut(&Point, &BoardCell) -> bool,
    );
}

impl <T: ReadableBoard> IterableBoard for T {
    /**
     * Allows us to easily iterate over a line of the board
     *
     * The iterator function can control if it want's to continue iterator
     * by returning a result. An Err will immediately end the iteration
     */
    fn for_each_until(
        &self,
        start: Point,
        dir: Direction,
        f: &mut dyn FnMut(&Point, &BoardCell) -> bool,
    ) {
        let mut loc = start.clone();

        loop {
            if let Some(bc) = self.get(&loc) {
                if !f(&loc, bc) {
                    return;
                }
            } else {
                return;
            }

            loc += &dir;
        }
    }

    fn for_each(
        &self,
        strip: &Strip,
        f: &mut dyn FnMut(&Point, &BoardCell) -> bool,
    ) {
        let mut loc = strip.start.clone();

        for _ in 0..strip.len {
            if let Some(bc) = self.get(&loc) {
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

        assert_eq!(board.get(&Point::new(0, 0)).unwrap(), &BoardCell::TripleWord);
        assert_eq!(board.get(&Point::new(3, 0)).unwrap(), &BoardCell::DoubleLetter);
        assert_eq!(board.get(&Point::new(1, 1)).unwrap(), &BoardCell::DoubleWord);
        assert_eq!(board.get(&Point::new(7, 7)).unwrap(), &BoardCell::StartingSpot);
        assert_eq!(board.get(&Point::new(BOARD_SIZE as i32, BOARD_SIZE as i32)), None);
    }

    #[test]
    fn set_and_get_tiles() {
        let mut board = Board::new();

        assert_eq!(
            board.set(&Point::new(0, 0), BoardCell::Tile(Tile::Letter('A'))).is_ok(),
            true
        );
        assert_eq!(board.get(&Point::new(0, 0)).unwrap(), &BoardCell::Tile(Tile::Letter('A')));

        assert_eq!(
            board.set(&Point::new(BOARD_SIZE as i32, BOARD_SIZE as i32), BoardCell::Empty).is_err(),
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
        board.set(&Point::new(1, 0), BoardCell::Tile(Tile::Letter('E'))).unwrap();
        board.set(&Point::new(3, 0), BoardCell::Tile(Tile::Letter('L'))).unwrap();
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

    fn make_board_with_overlay() -> Result<BoardWithOverlay> {
        /*
         * We construct a board that looks like this (in the top left corner)
         * .P.C..
         * .R.R..
         * ....D. <-- We play MINED here
         * .M.E..
         * .E.K..
         */
        let mut board = Board::new();
        board.set(&Point::new(1, 0), BoardCell::Tile(Tile::from('P')))?;
        board.set(&Point::new(1, 1), BoardCell::Tile(Tile::from('R')))?;
        // board.set(&Point::new(1, 2), BoardCell::Tile(Tile::from('I')))?;
        board.set(&Point::new(1, 3), BoardCell::Tile(Tile::from('M')))?;
        board.set(&Point::new(1, 4), BoardCell::Tile(Tile::from('E')))?;

        board.set(&Point::new(3, 0), BoardCell::Tile(Tile::from('C')))?;
        board.set(&Point::new(3, 1), BoardCell::Tile(Tile::from('R')))?;
        // board.set(&Point::new(3, 2), BoardCell::Tile(Tile::from('E')))?;
        board.set(&Point::new(3, 3), BoardCell::Tile(Tile::from('E')))?;
        board.set(&Point::new(3, 4), BoardCell::Tile(Tile::from('K')))?;

        board.set(&Point::new(4, 2), BoardCell::Tile(Tile::from('D')))?;

        let board_overlay = BoardWithOverlay::try_overlay(
            board,
            Point::new(0, 2),
            Direction::right(),
            "MINED"
        )?;

        Ok(board_overlay)
    }

    #[test]
    fn get_overlay_at() -> Result<()> {
        let board = make_board_with_overlay()?;

        assert_eq!(
            board.get_overlay_at(&Point::new(0, 2)),
            Some(&BoardCell::Tile(Tile::from('M')))
        );

        assert_eq!(
            board.get_overlay_at(&Point::new(1, 2)),
            Some(&BoardCell::Tile(Tile::from('I')))
        );

        assert_eq!(
            board.get_overlay_at(&Point::new(3, 2)),
            Some(&BoardCell::Tile(Tile::from('E')))
        );

        assert_eq!(board.get_overlay_at(&Point::new(0, 1)), None);
        assert_eq!(board.get_overlay_at(&Point::new(4, 3)), None);
        assert_eq!(board.get_overlay_at(&Point::new(2, 4)), None);
        Ok(())
    }

    #[test]
    fn full_overlay_test() -> Result<()> {
        let board_overlay = make_board_with_overlay()?;

        let (main_word, perp_words) = board_overlay.get_formed_words();

        assert_eq!(main_word.len(), 5);
        assert_eq!(perp_words.len(), 2);

        Ok(())
    }

    #[test]
    fn apply_to_board() -> Result<()> {
        let board_overlay = make_board_with_overlay()?;

        let board = board_overlay.apply_to_board();

        assert_eq!(board.get(&Point::new(0, 2)).unwrap(), &BoardCell::Tile(Tile::from('M')));
        assert_eq!(board.get(&Point::new(1, 2)).unwrap(), &BoardCell::Tile(Tile::from('I')));
        assert_eq!(board.get(&Point::new(2, 2)).unwrap(), &BoardCell::Tile(Tile::from('N')));
        assert_eq!(board.get(&Point::new(3, 2)).unwrap(), &BoardCell::Tile(Tile::from('E')));
        assert_eq!(board.get(&Point::new(4, 2)).unwrap(), &BoardCell::Tile(Tile::from('D')));

        Ok(())
    }
}
