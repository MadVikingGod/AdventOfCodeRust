use advent_of_code::util::Point;
use advent_of_code::y2021::day05::read_input;

use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
    let mut board: HashMap<Point, i64> = HashMap::new();
    let lines = read_input();

    lines
        .iter()
        .filter(|line| line.is_vertical() || line.is_horizontal())
        .for_each(|line| {
            line.iter().for_each(|point| {
                *board.entry(point).or_insert(0) += 1;
            })
        });

    println!("{}", board.iter().filter(|(_, &v)| v > 1).count());
    // println!("{:?}", board)

    let mut board2: HashMap<Point, i64> = HashMap::new();

    lines.iter().for_each(|line| {
        line.iter().for_each(|point| {
            *board2.entry(point).or_insert(0) += 1;
        })
    });
    println!("{}", board2.iter().filter(|(_, &v)| v > 1).count());
}
