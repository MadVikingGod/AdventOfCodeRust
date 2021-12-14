// Common tools
// use advent_of_code::util::Point;
// use advent_of_code::util::Field;
use std::collections::HashMap;
// use std::collections::HashSet;
// use std::collections::VecDeque;
// use itertools::Itertools;
use std::iter::FromIterator;

pub fn read_input() -> (String, HashMap<String, String>) {
    let input = include_str!("input.txt");
    parse_input(input)
}

pub fn parse_input(input: &str) -> (String, HashMap<String, String>) {
    let (template, map) = input.split_once("\n\n").unwrap();
    let map: HashMap<String,String> = HashMap::from_iter(
        map.lines().map(|line| {
            let (start,end) = line.split_once(" -> ").unwrap();
            let mut out =  start.to_string();
            out.replace_range(1..,end);
            (start.to_string(),out)
        }).collect::<Vec<(String,String)>>()
    );
    (template.to_string(), map)
}


pub fn read_adv_input() -> (Formula, Map) {
    let input = include_str!("input.txt");
    parse_adv_input(input)
}

#[derive(Debug)]
pub struct Formula {
    f: HashMap<String,u64>,
}
pub type Map = HashMap<String,Vec<String>>;

impl Formula {
    pub fn new() -> Formula {
        Formula{f: HashMap::new()}
    }
    pub fn insert(&mut self, s: String, v: u64) {
        self.f.insert(s, v);
    }
    pub fn inc(&mut self, s: String) {
        *self.f.entry(s).or_insert(0) += 1;
    }
    pub fn step(&self, map: &Map) -> Formula{
        let mut out = Formula::new();
        self.f.iter().for_each(|(k,v)| {
            match map.get(k) {
                Some(e) => {
                    e.iter().for_each(|nk| {
                        *out.f.entry(nk.to_string()).or_insert(0) += v
                    })
                },
                None => *out.f.entry(k.to_string()).or_insert(0) += v
            }
        });
        out
    }

    pub fn to_count(&self) -> HashMap<char, u64> {
        let mut count: HashMap<char, u64> = HashMap::new();
        self.f.iter().for_each(|(f,v)|{
            f.chars().take(1).for_each(|c| {
                *count.entry(c).or_insert(0) += v;
            });
        });
        count
    }
}

pub fn parse_adv_input(input: &str) -> (Formula, Map) {
    let (template, map) = input.split_once("\n\n").unwrap();
    let n1 = template.chars().take(template.len() - 1);
    let n2 = template.chars().skip(1);
    let mut template: Formula = Formula::new();
    n1.zip(n2).map(|(c1,c2)| 
        {[c1,c2].iter().collect::<String>()}
    ).for_each(|s| template.inc(s));

    let map: Map = HashMap::from_iter(
        map.lines().map(|line| {
            let (start,end) = line.split_once(" -> ").unwrap();
            let mut out1 =  start.to_string();
            out1.replace_range(1..,end);
            let mut out2 = start.to_string();
            out2.replace_range(..1, end);
            (start.to_string(),vec![out1,out2])
        }).collect::<Vec<(String,Vec<String>)>>()
    );
    (template, map)
}