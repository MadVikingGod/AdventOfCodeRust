// Common tools
use crate::util::Field;
use crate::util::Point;
// use std::collections::HashMap;
// use std::collections::HashSet;
// use std::collections::VecDeque;
// use itertools::Itertools;

pub fn read_input() -> Field<u32> {
    let input = include_str!("input.txt");
    parse(input)
}

pub fn parse(input: &str) -> Field<u32> {
    let mut f: Field<u32> = Field::new();
    input.lines().enumerate().for_each(|(y, line)| {
        let y = y as i64;
        line.chars().enumerate().for_each(|(x, c)| {
            let x = x as i64;
            f.insert(Point { x, y }, c.to_digit(10).unwrap());
        });
    });
    f
}
