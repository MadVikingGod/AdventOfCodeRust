use advent_of_code::y2021::day12::*;
// Common tools
// use advent_of_code::util::Point;
// use advent_of_code::util::Field;
// use std::collections::HashMap;
// use std::collections::HashSet;
use std::collections::VecDeque;
// use itertools::Itertools;

// const TEST_INPUT: &str = "start-A
// start-b
// A-c
// A-b
// b-d
// A-end
// b-end"; //10 + 36

const TEST_INPUT: &str = "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc"; //19 + 103

// const TEST_INPUT: &str = "fs-end
// he-DX
// fs-he
// start-DX
// pj-DX
// end-zg
// zg-sl
// zg-pj
// pj-he
// RW-he
// fs-DX
// pj-RW
// zg-RW
// start-pj
// he-WI
// zg-he
// pj-fs
// start-RW"; //226 + 3509

fn main() {
    println!("Hello, world!");

    let g = Graph::new(TEST_INPUT);
    println!("{:?}", g);
    let paths = find_paths(&g);
    println!("{}", paths.len());

    let g = read_input();
    println!("{:?}", g);
    let paths = find_paths(&g);
    println!("{}", paths.len());

    let g = Graph::new(TEST_INPUT);
    println!("{:?}", g);
    let paths = find_adv_paths(&g);
    println!("{}", paths.len());
    
    let g = read_input();
    println!("{:?}", g);
    let paths = find_adv_paths(&g);
    println!("{}", paths.len());
}

type Path = Vec<String>;

fn find_paths(g: &Graph) -> Vec<Path> {
    let mut q: VecDeque<(String, Path)> = VecDeque::new();
    q.push_front(("start".to_string(), vec!["start".to_string()]));
    let mut paths: Vec<Path> = vec![];

    while let Some((node, path)) = q.pop_front() {
        let neighbors = g.get(&node).unwrap();
        neighbors.iter()
            .cloned()
            .for_each(|neighbor| {
                let mut p = path.clone();
                p.push(neighbor.clone());
                if neighbor == "end" {
                    paths.push(p);
                    return
                }
                if is_lower(&neighbor) && path.contains(&neighbor) {
                    return
                }
                q.push_back((neighbor.clone(), p));
        });
        
    };
    paths
}

#[derive(Clone)]
struct Adv_Path {
    path: Vec<String>,
    has_double: bool,
}

fn is_lower(s: &String) -> bool {
    s == &s.to_lowercase()
}

impl Adv_Path {
    fn push(&mut self, s: String) {
        if is_lower(&s) && self.path.contains(&s) {
            self.has_double = true;
        };
        self.path.push(s);
    }
    fn can_push(&self, s:&String) -> bool {
        if s == "start" {
            return false
        }
        if !is_lower(s) {
            return true
        }
        !(self.has_double && self.path.contains(s))
    }
}


fn find_adv_paths(g: &Graph) -> Vec<Adv_Path> {
    let mut q: VecDeque<(String, Adv_Path)> = VecDeque::new();
    let path = Adv_Path{path: vec!["start".to_string()], has_double:false};
    q.push_front(("start".to_string(), path));
    let mut paths: Vec<Adv_Path> = vec![];

    while let Some((node, path)) = q.pop_front() {
        let neighbors = g.get(&node).unwrap();
        neighbors.iter()
            .cloned()
            .for_each(|neighbor| {
                let mut p = path.clone();
                if neighbor == "end" {
                    p.push(neighbor.clone());
                    paths.push(p);
                    return
                }
                if !path.can_push(&neighbor) {
                    return
                }
                p.push(neighbor.clone());
                q.push_back((neighbor.clone(), p));
        });
        
    };
    paths
}