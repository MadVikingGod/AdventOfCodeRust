use super::point::*;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Field<T> {
    pub map: HashMap<Point, T>,
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

    pub fn neighbor_values(&self, p: &Point) -> Vec<T> {
        self.neighbors(p).iter().map(|(_, v)| *v).collect()
    }

    pub fn get(&self, p: &Point) -> Option<&T> {
        self.map.get(p)
    }
    pub fn len(&self) -> usize {
        self.map.len()
    }
}
