use super::{Point, Direction};

pub struct Line {
    start: Point,
    direction: Direction,
    length: i32
}

impl Line {
    pub fn new(start: Point, direction: Direction, length: i32) -> Self {
        Self { start, direction, length}
    }

    pub fn get_end(&self) -> Point {
        Point::new(
            self.start.x() + self.direction.x() * (self.length - 1),
            self.start.y() + self.direction.y() * (self.length - 1)
        )
    }

    pub fn includes(&self, point: &Point) -> bool {
        let start = self.start;
        let end = self.get_end();

        start.x() <= point.x() && end.x() >= point.x() &&
        start.y() <= point.y() && end.y() >= point.y()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_includes() {
        let line = Line::new(
            Point::new(5, 10),
            Direction::new(0, 1),
            10
        );

        assert_eq!(line.includes(&Point::new(5, 15)), true);
        assert_eq!(line.includes(&Point::new(5, 19)), true);
        assert_eq!(line.includes(&Point::new(5, 20)), false);
        assert_eq!(line.includes(&Point::new(6, 15)), false);
        assert_eq!(line.includes(&Point::new(6, 19)), false);
        assert_eq!(line.includes(&Point::new(6, 20)), false);
    }
}
