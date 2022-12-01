use advent_of_code::y2022::day01::*;
// Common tools
// use advent_of_code::util::Field;
// use advent_of_code::util::Point;
// use std::collections::HashMap;
// use std::collections::HashSet;
// use std::collections::VecDeque;
// use itertools::Itertools;

fn main() {
    let mut elfs = read_input();
    println!("Part 1: {}", elfs.iter().max().unwrap());
    elfs.sort();
    println!("Part 2: {}", elfs.iter().rev().take(3).sum::<i64>());
}
