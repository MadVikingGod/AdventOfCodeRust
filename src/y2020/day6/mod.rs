use std::collections::HashSet;

pub fn read_input() -> Vec<HashSet<char>> {
    let input = include_str!("input.txt");
    input.split("\n\n").map(convert).collect()
}

pub fn read_input2() -> Vec<HashSet<char>> {
    let input = include_str!("input.txt");
    input.split("\n\n").map(convert_everyone).collect()
}

pub fn convert(s: &str) -> HashSet<char> {
    s.chars().filter(|c| c.is_ascii_lowercase()).collect()
}

pub fn convert_everyone(s: &str) -> HashSet<char> {
    let members: Vec<HashSet<char>> = s
        .lines()
        .map(|l| l.chars().collect::<HashSet<char>>())
        .collect();
    if members.len() == 0 {
        return HashSet::new();
    }
    let mut u = members.get(0).unwrap().clone();
    for b in members {
        u = u.intersection(&b).copied().collect();
    }
    u
}
