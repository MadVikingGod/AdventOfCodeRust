use advent_of_code::y2021::day9::*;
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
    let heights: i64 = map.iter().filter_map(|(p, v)| if map.neighbor_values(*p).iter().all(|n_v| v< n_v) {Some(1+v)} else {None}).sum();
    println!("ex heights: {:?}", heights);
    let low_points: Vec<Point> =  map.iter().filter(|(&p, &v)| map.neighbor_values(p).iter().all(|&n_v| v< n_v)).map(|(&p,_)| p).collect();
    let score: usize = low_points.iter().map(|p| find_basin(p, &map)).sorted().rev().take(3).product();
    println!("ex score  : {:?}", score);

    let map = read_input();
    let heights: i64 = map.iter().filter_map(|(p, v)| if map.neighbor_values(*p).iter().all(|n_v| v< n_v) {Some(1+v)} else {None}).sum();
    println!("{:?}", heights);
    let low_points: Vec<Point> =  map.iter().filter(|(&p, &v)| map.neighbor_values(p).iter().all(|&n_v| v< n_v)).map(|(&p,_)| p).collect();
    let score: usize = low_points.iter().map(|p| find_basin(p, &map)).sorted().rev().take(3).product();
    println!("{:?}", score);
}


fn find_basin(p: &Point, m: &Map) -> usize {
    let mut seen: HashSet<Point> = HashSet::new();
    let mut q: VecDeque<Point> = VecDeque::from([*p]);

    while let Some(p) = q.pop_front() {
        seen.insert(p);
        let v = m.get(&p).unwrap();
        m.neighbors(p).iter()
            .filter(|(n_p, n_v)| *n_v > *v && *n_v!=9 && !seen.contains(n_p))
            .for_each(|(n_p,_)| q.push_back(*n_p));

    };
    seen.len()
}