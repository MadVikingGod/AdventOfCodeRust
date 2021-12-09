use std::collections::HashMap;

pub fn read_input() -> Map {
    let input = include_str!("input.txt");
    parse_map(input)
}

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
impl std::ops::Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

const NORTH: Point = Point { x: 0, y: -1 };
const SOUTH: Point = Point { x: 0, y: 1 };
const EAST: Point = Point { x: 1, y: 0 };
const WEST: Point = Point { x: -1, y: 0 };

const DIRECTIONS: [Point; 4] = [NORTH, SOUTH, EAST, WEST];

#[derive(Debug, Clone)]
pub struct Map {
    pub map: HashMap<Point, i64>,
}

impl Map {
    pub fn iter(&self) -> std::collections::hash_map::Iter<'_, Point, i64> {
        self.map.iter()
    }

    pub fn neighbors(&self, p: Point) -> Vec<(Point, i64)> {
        DIRECTIONS
            .iter()
            .filter_map(|&dir| Some((dir + p, *self.map.get(&(dir + p))?)))
            .collect()
    }

    pub fn neighbor_values(&self, p: Point) -> Vec<i64> {
        self.neighbors(p).iter().map(|(_, v)| *v).collect()
    }

    pub fn get(&self, p: &Point) -> Option<&i64> {
        self.map.get(p)
    }
}

pub fn parse_map(input: &str) -> Map {
    let mut map = Map {
        map: HashMap::new(),
    };
    input.lines().enumerate().for_each(|(y, line)| {
        let y = y as i64;
        line.chars().enumerate().for_each(|(x, c)| {
            let x = x as i64;
            map.map
                .insert(Point { x, y }, c.to_digit(10).unwrap() as i64);
        });
    });
    map
}
