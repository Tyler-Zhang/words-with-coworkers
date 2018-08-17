pub struct Direction(i32, i32);

impl Direction {
    #[inline]
    pub fn new(x: i32, y: i32) -> Self {
        Direction(x, y)
    }

    #[inline]
    pub fn x(&self) -> i32 {
        self.0
    }

    #[inline]
    pub fn y(&self) -> i32 {
        self.1
    }

    #[inline]
    pub fn opposite(&self) -> Self {
        Direction(self.0 * -1, self.1 * -1)
    }
}

impl From<(i32, i32)> for Direction {
    fn from(pair: (i32, i32)) -> Self {
        Direction(pair.0, pair.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub fn test_opposite() {
        let direction = Direction::new(5, 10);

        assert_eq!(direction.x(), 5);
        assert_eq!(direction.y(), 10);

        let opposite = direction.opposite();

        assert_eq!(opposite.x(), -5);
        assert_eq!(opposite.y(), -10);
    }
}
