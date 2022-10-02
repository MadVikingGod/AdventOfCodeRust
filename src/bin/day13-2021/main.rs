use advent_of_code::y2021::day13::*;
// Common tools
// use advent_of_code::util::Point;
// use advent_of_code::util::Field;
// use std::collections::HashMap;
// use std::collections::HashSet;
// use std::collections::VecDeque;
// use itertools::Itertools;

fn main() {
    println!("Hello, world!");

    let p = Page::new(
        "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0",
    );

    println!("{}", p.fold_y(7).as_field());
    println!("{}", p.fold_y(7).fold_x(5).as_field());

    let (p, _) = read_input();
    println!("{}", p.fold_x(655).len());
    println!(
        "{}",
        p.fold_x(655)
            .fold_y(447)
            .fold_x(327)
            .fold_y(223)
            .fold_x(163)
            .fold_y(111)
            .fold_x(81)
            .fold_y(55)
            .fold_x(40)
            .fold_y(27)
            .fold_y(13)
            .fold_y(6)
            .as_field()
    )
}
