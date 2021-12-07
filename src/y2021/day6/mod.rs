use std::collections::HashMap;
pub fn read_input() -> School {
    let input = include_str!("input.txt");
    parse_school(input)
}

pub fn parse_school(input: &str) -> School {
    let mut fish: HashMap<u64, u64> = HashMap::new();
    input
        .split(",")
        .map(|val| val.parse::<u64>().unwrap())
        .for_each(|f| *fish.entry(f).or_insert(0) += 1);
    School { fish: fish }
}

#[derive(Debug, Clone, PartialEq)]
pub struct School {
    pub fish: HashMap<u64, u64>,
}

impl School {
    pub fn count_fish(self: &Self) -> u64 {
        self.fish.values().sum()
    }
}

impl Iterator for School {
    type Item = School;
    fn next(&mut self) -> Option<Self::Item> {
        let mut next: HashMap<u64, u64> = HashMap::new();
        self.fish.iter_mut().for_each(|(k, v)| {
            match k {
                0 => {
                    next.insert(8, *v);
                    *next.entry(6).or_insert(0) += *v;
                }
                _ => *next.entry(k - 1).or_insert(0) += *v,
            };
        });
        self.fish = next.clone();
        Some(School { fish: next })
    }
}
