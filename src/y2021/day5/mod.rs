use crate::util::Point;
use itertools::Itertools;
use std::cmp::Ordering;

pub fn read_input() -> Vec<Line> {
    let input = include_str!("input.txt");

    parse_lines(input.lines().collect())
}

pub fn parse_lines(lines: Vec<&str>) -> Vec<Line> {
    lines
        .iter()
        .map(|line| {
            let (p1, p2) = line
                .split(" -> ")
                .map(|point| {
                    let (v1, v2) = point
                        .split(',')
                        .map(|vert| vert.parse().unwrap())
                        .next_tuple()
                        .unwrap();
                    Point { x: v1, y: v2 }
                })
                .next_tuple()
                .unwrap();
            Line { p1, p2 }
        })
        .collect()
}

#[derive(Debug, Clone, PartialEq)]
pub struct Line {
    pub p1: Point,
    pub p2: Point,
}

impl Line {
    pub fn is_vertical(&self) -> bool {
        self.p1.y == self.p2.y
    }

    pub fn is_horizontal(&self) -> bool {
        self.p1.x == self.p2.x
    }

    pub fn slope(&self) -> f64 {
        (self.p1.y - self.p2.y) as f64 / (self.p1.x - self.p2.x) as f64
    }

    pub fn iter(&self) -> LineIter {
        let x = match self.p2.x.cmp(&self.p1.x) {
            Ordering::Greater => 1,
            Ordering::Less => -1,
            Ordering::Equal => 0,
        };

        let y = match self.p2.y.cmp(&self.p1.y) {
            Ordering::Greater => 1,
            Ordering::Less => -1,
            Ordering::Equal => 0,
        };
        LineIter {
            cur: self.p1.clone(),
            stop: self.p2.clone(),
            dir: Point { x, y },
            finished: false,
        }
    }
}

pub struct LineIter {
    cur: Point,
    stop: Point,
    dir: Point,
    finished: bool,
}

impl Iterator for LineIter {
    type Item = Point;
    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        };
        let ret = self.cur.clone();
        if self.cur == self.stop {
            self.finished = true
        };
        self.cur = self.cur + self.dir;
        Some(ret)
    }
}
