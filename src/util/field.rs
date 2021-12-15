use super::point::*;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Field<T> {
    pub map: HashMap<Point, T>,
}

impl<T> Default for Field<T>
where
    T: Copy,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Field<T>
where
    T: Copy,
{
    pub fn new() -> Field<T> {
        Field {
            map: HashMap::new(),
        }
    }

    pub fn iter(&self) -> std::collections::hash_map::Iter<'_, Point, T> {
        self.map.iter()
    }

    pub fn insert(&mut self, p: Point, v: T) -> Option<T> {
        self.map.insert(p, v)
    }

    pub fn neighbors(&self, p: &Point) -> Vec<(Point, T)> {
        DIRECTIONS
            .iter()
            .filter_map(|dir| Some((dir + p, *self.map.get(&(dir + p))?)))
            .collect()
    }
    pub fn neighbors_diag(&self, p: &Point) -> Vec<(Point, T)> {
        DIRECTIONS_DIAG
            .iter()
            .filter_map(|dir| Some((dir + p, *self.map.get(&(dir + p))?)))
            .collect()
    }

    pub fn neighbor_values(&self, p: &Point) -> Vec<T> {
        self.neighbors(p).iter().map(|(_, v)| *v).collect()
    }

    pub fn get(&self, p: &Point) -> Option<&T> {
        self.map.get(p)
    }
    pub fn len(&self) -> usize {
        self.map.len()
    }
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
    pub fn contains(&self, p: &Point) -> bool {
        self.map.contains_key(p)
    }
}

impl<T> fmt::Display for Field<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut min_x = 0;
        let mut max_x = 0;
        let mut min_y = 0;
        let mut max_y = 0;
        self.map.keys().for_each(|p| {
            if p.x > max_x {
                max_x = p.x
            };
            if p.x < min_x {
                min_x = p.x
            };
            if p.y > max_y {
                max_y = p.y
            };
            if p.y < min_y {
                min_y = p.y
            };
        });
        for y in min_y..max_y + 1 {
            for x in min_x..max_x + 1 {
                match self.map.get(&Point { x, y }) {
                    Some(n) => write!(f, "{}", n)?,
                    None => write!(f, ".")?,
                };
            }
            if y != max_y {
                writeln!(f)?
            };
        }
        Ok(())
    }
}

#[test]
fn test_display() {
    let f: Field<char> = Field {
        map: HashMap::from([
            (Point { x: 0, y: 3 }, '3'),
            (Point { x: 4, y: 1 }, ']'),
            (Point { x: 2, y: 2 }, '!'),
        ]),
    };
    let want = ".....
....]
..!..
3....";
    assert_eq!(format!("{}", f), want)
}
