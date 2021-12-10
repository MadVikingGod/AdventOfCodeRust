use advent_of_code::y2021::day9::*;
use advent_of_code::util::Point;
use advent_of_code::util::Field;
use std::collections::HashSet;
use std::collections::VecDeque;
use itertools::Itertools;

fn main() {
    println!("Hello, world!");

    let input = "2199943210
3987894921
9856789892
8767896789
9899965678";
    let map = parse_map(input);
    let low_points = find_low_points(&map);
    let heights: i64 = low_points.iter().map(|(_,v)| v+1).sum();
    println!("ex heights: {:?}", heights);
    let score: usize = low_points.iter().map(|(p,_)| find_basin(p, &map)).sorted().rev().take(3).product();
    println!("ex score  : {:?}", score);

    let map = read_input();
    let low_points = find_low_points(&map);
    let heights: i64 = low_points.iter().map(|(_,v)| v+1).sum();
    println!("{:?}", heights);
    let score: usize = low_points.iter().map(|(p,_)| find_basin(p, &map)).sorted().rev().take(3).product();
    println!("{:?}", score);
}

fn find_low_points(m: &Field<i64>) -> Vec<(Point, i64)> {
    m.iter()
        .filter_map(|(p, v)| {
            if m.neighbor_values(p).iter().all(|n_v| v< n_v) 
                {Some((*p, *v))}
            else
                {None}
        }).collect()
}


fn find_basin(p: &Point, m: &Field<i64>) -> usize {
    let mut seen: HashSet<Point> = HashSet::new();
    let mut q: VecDeque<Point> = VecDeque::from([*p]);

    while let Some(p) = q.pop_front() {
        seen.insert(p);
        let v = m.get(&p).unwrap();
        m.neighbors(&p).iter()
            .filter(|(n_p, n_v)| *n_v > *v && *n_v!=9 && !seen.contains(n_p))
            .for_each(|(n_p,_)| q.push_back(*n_p));

    };
    seen.len()
}