use advent_of_code::y2021::day15::*;
// Common tools
// use advent_of_code::util::Field;
use advent_of_code::util::Point;
// use std::collections::HashMap;
// use std::collections::HashSet;
// use std::collections::VecDeque;
// use itertools::Itertools;

const TEST_INPUT: &str ="1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

fn main() {
    println!("Hello, world!");

    let f = parse_input(TEST_INPUT);
    println!("{}", short_path(&f, &Point{x:9,y:9}));

    let f = read_input();
    println!("{}", short_path(&f, &Point{x:99,y:99}));

    let f = parse_adv_input(TEST_INPUT);
    println!("{}", short_path(&f, &Point{x:49,y:49}));

    let f = read_adv_input();
    println!("{}", short_path(&f, &Point{x:499,y:499}));

}
