use advent_of_code::y2021::day18::*;
// Common tools
// use advent_of_code::util::Field;
// use advent_of_code::util::Point;
// use std::collections::HashMap;
// use std::collections::HashSet;
// use std::collections::VecDeque;
// use itertools::Itertools;

fn main() {
    println!("Hello, world!");

    let input = read_input();
    let output = add_all(input);
    println!("{}", output);
    println!("{}", output.magnitude());

    let input = read_input();
    let output = add_pairs(input);
    println!("{}", output)

    //10541 is too high
}
