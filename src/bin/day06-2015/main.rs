use advent_of_code::y2015::day06::*;
// Common tools
// use advent_of_code::util::Field;
// use advent_of_code::util::Point;
// use std::collections::HashMap;
// use std::collections::HashSet;
// use std::collections::VecDeque;
// use itertools::Itertools;

fn main() {
    println!("Hello, world!");

    let instructions = read_input();
    let mut lights: Lights = [[false; 1000]; 1000];

    for inst in &instructions {
        inst.apply(&mut lights)
    }

    println!("{}", count(&lights));

    let mut lights: DimLights = [[0; 1000]; 1000];

    for inst in instructions {
        inst.applyDim(&mut lights)
    }

    println!("{}", count_dim(&lights));
}
