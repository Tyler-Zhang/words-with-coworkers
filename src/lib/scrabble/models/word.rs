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

    pub fn get_end(&self) -> Point {
        self.start + (self.direction * ((self.letters.len() - 1usize) as i32))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_end() {
        let word = Word::new(format!("HELLO"), (1, 5), true);
        let end = word.get_end();

        assert_eq!(end.x(), 1);
        assert_eq!(end.y(), 9);
    }
}
