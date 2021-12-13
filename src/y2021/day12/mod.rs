// Common tools
// use advent_of_code::util::Point;
// use advent_of_code::util::Field;
use std::collections::HashMap;
use std::collections::HashSet;
// use std::collections::VecDeque;
// use itertools::Itertools;

pub fn read_input() -> Graph {
    let input = include_str!("input.txt");
    Graph::new(input)
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Node {
    pub edges: HashSet<String>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Graph {
    pub nodes: HashMap<String, Node>
}

impl Graph {
    pub fn new(input: &str) -> Graph {
        let mut g = Graph{nodes: HashMap::new()};
        input.lines().for_each(|line| {
            let (n1, n2) = line.split_once("-").unwrap();
            g.insert(n1.to_string(), n2.to_string())
        });
        g
    }

    pub fn insert(&mut self, n1: String, n2: String) {
        self.nodes.entry(n1.clone()).or_insert(Node{edges: HashSet::new()}).edges.insert(n2.clone());
        self.nodes.entry(n2).or_insert(Node{edges: HashSet::new()}).edges.insert(n1);
    }
    pub fn get(&self, node: &String) -> Option<Vec<String>> {
        let n = self.nodes.get(node)?;
        Some(n.edges.iter().cloned().collect())
    }
}