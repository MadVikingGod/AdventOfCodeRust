use advent_of_code::y2021::day11::*;
// Common tools
use advent_of_code::util::Point;
use advent_of_code::util::Field;
// use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
// use itertools::Itertools;

const TEST_INPUT: &str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

fn main() {
    println!("Hello, world!");
    let mut f = read_input();
    let mut count: u64 = 0;
    for _ in 0..100 {
        f = step(&f);
        f.iter().for_each(|(_,v)| if *v == 0 { count += 1 })
    }
    println!("{}", f);
    println!("{}", count);

    let mut steps = 100;
    while !all_zero(&f) {
        f = step(&f);
        steps += 1;
    }
    println!("{}", steps);

}

fn all_zero(f: &Field<u32>) -> bool {
    f.iter().all(|(_,v)| *v==0)
}

fn step(f: &Field<u32>) -> Field<u32> {
    let mut flashed: HashSet<Point> = HashSet::new();
    let mut q: VecDeque<Point> = VecDeque::new();
    let mut next = f.clone();
    for (p,v) in next.map.iter_mut() {
        *v += 1;
        if *v > 9 {
            flashed.insert(p.clone());
            q.push_back(p.clone());
        };
    };

    while let Some(p) = q.pop_front() {
        next.neighbors_diag(&p).iter().for_each(|(np, v)| {
            next.insert(*np, v+1);
            if v+1 >9 && !flashed.contains(np) {
                q.push_back(np.clone());
                flashed.insert(*np);
            }
        })
    }

    for p in flashed.iter() {
        next.insert(*p, 0);
    }

    next
}