// Common tools
use crate::util::Field;
use crate::util::Point;
// use std::collections::HashMap;
// use std::collections::HashSet;
// use std::collections::VecDeque;
// use itertools::Itertools;
use itertools::iproduct;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn read_input() -> Field<u64> {
    let input = include_str!("input.txt");
    parse_input(input)
}

pub fn read_adv_input() -> Field<u64> {
    let input = include_str!("input.txt");
    parse_adv_input(input)
}

pub fn parse_input(input: &str) -> Field<u64> {
    let mut f: Field<u64> = Field::new();
    input.lines().enumerate().for_each(|(y, line)| {
        let y = y as i64;
        line.chars().enumerate().for_each(|(x, c)| {
            let x = x as i64;
            f.insert(
                Point {
                    x: x.into(),
                    y: y.into(),
                },
                c.to_digit(10).unwrap() as u64,
            );
        });
    });
    f
}

pub fn parse_adv_input(input: &str) -> Field<u64> {
    let width = input.lines().count();
    let mut f: Field<u64> = Field::new();
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            for (xm, ym) in iproduct!(0..5, 0..5) {
                let x = (x + width * xm) as i64;
                let y = (y + width * ym) as i64;
                let digit = c.to_digit(10).unwrap() as usize;
                let mut out = digit + xm + ym;
                if out > 9 {
                    out -= 9
                };
                f.insert(Point { x: x, y: y }, out as u64);
            }
        });
    });
    f
}

#[derive(Debug, Clone)]
pub struct Path {
    score: u64,
    point: Point,
}
impl Ord for Path {
    fn cmp(&self, other: &Self) -> Ordering {
        Reverse(self.score).cmp(&Reverse(other.score))
    }
}
impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Path {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}
impl Eq for Path {}

pub fn short_path(f: &Field<u64>, dest: &Point) -> u64 {
    let mut q: BinaryHeap<Path> = BinaryHeap::new();
    let start = Point { x: 0, y: 0 };
    q.push(Path {
        score: *f.get(&start).unwrap_or(&0),
        point: start,
    });
    let mut seen: Field<u64> = Field::new();

    while let Some(current) = q.pop() {
        if seen.contains(&current.point) {
            continue;
        }
        seen.insert(current.point, current.score);
        let neighbors = f.neighbors(&current.point);

        for (p, v) in neighbors.iter().filter(|&(n, _)| !seen.contains(n)) {
            if p == dest {
                return current.score + v;
            };
            q.push(Path {
                score: current.score + v,
                point: *p,
            });
        }
    }
    0
}
