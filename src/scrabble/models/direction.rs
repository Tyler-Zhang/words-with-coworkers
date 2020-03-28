use std::ops;

pub static DIRECTION_DOWN: Direction = Direction::new(0, 1);
pub static DIRECTION_RIGHT: Direction = Direction::new(1, 0);
pub static DIRECTION_LEFT: Direction = Direction::new(-1, 0);
pub static DIRECTION_UP: Direction = Direction::new(0, -1);

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
}

impl ops::Add<Direction> for Point {
    type Output = Self;

    fn add(self, rhs: Direction) -> Self {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::AddAssign<&Direction> for Point {
    fn add_assign(&mut self, other: &Direction) {
        self.x += other.x;
        self.y += other.y;
    }
}

#[derive(Clone, Copy)]
pub struct Direction {
    pub x: i32,
    pub y: i32,
}

impl Direction {
    pub fn new(x: i32, y: i32) -> Direction {
        Direction { x, y }
    }

    pub fn is_horizontal(&self) -> bool {
        self.x != 0
    }

    pub fn is_vertical(&self) -> bool {
        self.y != 0
    }
}

impl ops::Mul<i32> for Direction {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self {
        Direction {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

/**
 * Represents a strip of tiles on the board
 */
pub struct Strip {
    pub start: Point,
    pub dir: Direction,
    pub len: i32
}

impl Strip {
    pub fn new(start: Point, dir: Direction, len: i32) -> Self {
        Self {
            start,
            dir,
            len
        }
    }

    #[inline]
    fn is_between(val: i32, a: i32, b: i32) -> bool{
        if a == b && val == a {
            true
        } else if a < b {
            (a..b).contains(&val)
        } else {
            ((b+1)..=a).contains(&val)
        }
    }

    pub fn contains(&self, point: &Point) -> bool {
        Self::is_between(point.x, self.start.x, self.start.x + self.dir.x * self.len) &&
        Self::is_between(point.y, self.start.y, self.start.y + self.dir.y * self.len)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strip_contains() {
        let s = Strip::new(
            Point::new(5, 5),
            Direction::new(1, 0),
            5
        );

        assert_eq!(s.contains(&Point::new(5, 5)), true);
        assert_eq!(s.contains(&Point::new(6, 5)), true);
        assert_eq!(s.contains(&Point::new(10, 5)), false);

        assert_eq!(s.contains(&Point::new(5, 6)), false);
        assert_eq!(s.contains(&Point::new(5, 4)), false);
    }
}
