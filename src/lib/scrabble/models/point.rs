use std::ops::{Add, AddAssign};
use super::Direction;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Point(i32, i32);

impl Point {
    #[inline]
    pub fn new(x: i32, y: i32) -> Self {
        Point(x, y)
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
        Point(self.0 * -1, self.1 * -1)
    }
}

impl From<(i32, i32)> for Point {
    fn from(pair: (i32, i32)) -> Self {
        Point(pair.0, pair.1)
    }
}

impl Add<Direction> for Point {
    type Output = Point;

    fn add(self, rhs: Direction) -> Self {
        Self::new(self.x() + rhs.x(), self.y() + rhs.y())
    }
}

impl AddAssign<Direction> for Point {
    fn add_assign(&mut self, rhs: Direction) {
        self.0 += rhs.x();
        self.1 += rhs.y();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

}
