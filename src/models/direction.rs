use std::ops;

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

    pub fn down() -> Direction { Direction::new(0, 1) }
    pub fn right() -> Direction { Direction::new(1, 0) }
    pub fn left() -> Direction { Direction::new(-1, 0) }
    pub fn up() -> Direction { Direction::new(0, -1) }
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
        if a < b {
            (a..b).contains(&val)
        } else {
            ((b+1)..=a).contains(&val)
        }
    }

    pub fn contains(&self, point: &Point) -> bool {
        (
            point.x == self.start.x ||
            Self::is_between(point.x, self.start.x, self.start.x + self.dir.x * self.len)
        ) && (
            point.y == self.start.y ||
            Self::is_between(point.y, self.start.y, self.start.y + self.dir.y * self.len)
        )
    }

    /**
     * Get the distance of the point from the start of the strip
     */
    pub fn distance_in(&self, point: &Point) -> Option<i32> {
        if !self.contains(point) {
            None
        } else {
            let dist = (point.x as i32 - self.start.x as i32).abs() +
                       (point.y as i32 - self.start.y as i32).abs();
            Some(dist)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strip_contains_horizontal() {
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

        assert_eq!(s.contains(&Point::new(4, 5)), false);
        assert_eq!(s.contains(&Point::new(3, 5)), false);
    }

    #[test]
    fn strip_contains_vertical_negative() {
        let s = Strip::new(
            Point::new(5, 5),
            Direction::new(0, -1),
            5
        );

        assert_eq!(s.contains(&Point::new(5, 5)), true);
        assert_eq!(s.contains(&Point::new(6, 5)), false);
        assert_eq!(s.contains(&Point::new(10, 5)), false);

        assert_eq!(s.contains(&Point::new(5, 4)), true);
        assert_eq!(s.contains(&Point::new(5, 1)), true);
        assert_eq!(s.contains(&Point::new(5, 0)), false);

        assert_eq!(s.contains(&Point::new(5, 6)), false);
    }
}
