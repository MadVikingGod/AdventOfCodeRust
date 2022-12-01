// Common tools
// use crate::util::Field;
// use crate::util::Point;
// use std::collections::HashMap;
// use std::collections::HashSet;
// use std::collections::VecDeque;
// use itertools::Itertools;

pub fn read_input() -> Vec<i64> {
    let input = include_str!("input.txt");
    parse_elfs(input)
}

pub fn parse_elfs(input: &str) -> Vec<i64> {
    input
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .map(|line| line.parse::<i64>().unwrap())
                .sum::<i64>()
        })
        .collect()
}
