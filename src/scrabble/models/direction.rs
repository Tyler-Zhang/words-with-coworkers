use std::ops;

#[derive(Clone)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

impl Point {
    pub fn new(x: u32, y: u32) -> Point {
        Point { x, y }
    }
}

#[derive(Clone)]
pub struct Direction {
    pub x: u32,
    pub y: u32,
}

impl Direction {
    pub fn new(x: u32, y: u32) -> Direction {
        Direction { x, y }
    }
}

impl ops::AddAssign<&Direction> for Point {
    fn add_assign(&mut self, other: &Direction) {
        self.x += other.x;
        self.y += other.y;
    }
}
