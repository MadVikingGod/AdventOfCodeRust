#[derive(Debug, Clone, Hash, Eq, PartialEq, Copy)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

impl std::ops::Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<'a, 'b> std::ops::Add<&'b Point> for &'a Point {
    type Output = Point;

    fn add(self, other: &'b Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::Mul<i64> for Point {
    type Output = Self;

    fn mul(self, rhs: i64) -> Self::Output {
        Point {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

pub const NORTH: Point = Point { x: 0, y: -1 };
pub const SOUTH: Point = Point { x: 0, y: 1 };
pub const EAST: Point = Point { x: 1, y: 0 };
pub const WEST: Point = Point { x: -1, y: 0 };

pub const DIRECTIONS: [Point; 4] = [NORTH, SOUTH, EAST, WEST];

pub const NE: Point = Point { x: 1, y: -1 };
pub const SE: Point = Point { x: 1, y: 1 };
pub const SW: Point = Point { x: -1, y: 1 };
pub const NW: Point = Point { x: -1, y: -1 };

pub const DIRECTIONS_DIAG: [Point; 8] = [NORTH, SOUTH, EAST, WEST, NE, SE, SW, NW];
