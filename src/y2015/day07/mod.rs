// Common tools
// use crate::util::Field;
// use crate::util::Point;
// use std::collections::HashMap;
// use std::collections::HashSet;
// use std::collections::VecDeque;
// use itertools::Itertools;

pub fn read_input() -> Vec<&'static str> {
    let input = include_str!("input.txt");
    input.lines().collect()
}
