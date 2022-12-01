use advent_of_code::y2021::day17::*;
// Common tools
// use advent_of_code::util::Field;
// use advent_of_code::util::Point;
// use std::collections::HashMap;
// use std::collections::HashSet;
// use std::collections::VecDeque;
// use itertools::Itertools;

//target area: x=153..199, y=-114..-75
fn main() {
    println!("Hello, world!");
    let steps = x_steps(153, 199);
    let vels = get_points(steps, -114, -75);
    println!("{}", vels.len())
}
