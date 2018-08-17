use super::{Point, Direction};

pub struct Word {
    pub letters: String,
    pub start: Point,
    pub direction: Direction,
    pub direction_down: bool,
}

impl Word {
    pub fn new(letters: String, start: (i32, i32), direction_down: bool) -> Word {
        Word {
            letters,
            start: Point::from(start),
            direction: (if direction_down { Direction::new(0, 1) } else { Direction::new(1, 0) }),
            direction_down
        }
    }
}
