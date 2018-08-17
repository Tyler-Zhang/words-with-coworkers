use super::Direction;

pub struct Word<'a> {
    pub letters: &'a str,
    pub start: (i32, i32),
    pub direction: Direction,
    pub direction_down: bool,
}

impl<'a> Word<'a> {
    pub fn new(letters: &'a str, start: (i32, i32), direction_down: bool) -> Word<'a> {
        Word {
            letters,
            start,
            direction: (if direction_down { Direction::new(0, 1) } else { Direction::new(1, 0) }),
            direction_down
        }
    }
}
