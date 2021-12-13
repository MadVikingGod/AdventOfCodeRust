// Common tools
use crate::util::Point;
use crate::util::Field;
// use std::collections::HashMap;
use std::collections::HashSet;
// use std::collections::VecDeque;
// use itertools::Itertools;

pub fn read_input() -> (Page,&'static str) {
    let input = include_str!("input.txt");
    let (page, inst) = input.split_once("\n\n").unwrap();
    (Page::new(page), inst)
}

pub struct Page {
    dots: HashSet<Point>,
}

impl Page {
    pub fn new(input: &str) -> Page {
        Page{dots: input.lines().map(|line| {
            let (x,y) = line.split_once(",").unwrap();
            Point{x:x.parse().unwrap(),y:y.parse().unwrap()}
        }).collect()}
    }
    pub fn fold_x(&self, fold: i64) -> Page {
        Page{
            dots: self.dots.iter().map(|p| {
                if p.x < fold {
                    return *p
                };
                Point{x: 2*fold - p.x, y: p.y}
            }).collect()
        }
    }
    pub fn fold_y(&self, fold: i64) -> Page {
        Page{
            dots: self.dots.iter().map(|p| {
                if p.y < fold {
                    return *p
                };
                Point{x: p.x, y: 2*fold - p.y}
            }).collect()
        }
    }
    pub fn len(&self) -> usize {
        self.dots.len()
    }

    pub fn as_field(&self) -> Field<char> {
        let mut f = Field::new();
        self.dots.iter().for_each(|p| {f.insert(*p, '#');});
        f
    }
}