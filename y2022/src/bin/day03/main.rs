use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt");
    let priority = input
        .lines()
        .map(|line| Rucksack::new(line.trim()).priority().unwrap() as u64)
        .sum::<u64>();
    println!("Part 1: {}", priority);

    let priority = input
        .lines()
        .collect::<Vec<_>>()
        .chunks_exact(3)
        .map(|v| Group::new(v).priority().unwrap() as u64)
        .sum::<u64>();
    println!("Part 2: {}", priority);
}

struct Rucksack {
    left: HashSet<u8>,
    right: HashSet<u8>,
}

impl Rucksack {
    fn new(input: &str) -> Self {
        let mid = input.len() / 2;
        Self {
            left: input.bytes().take(mid).collect(),
            right: input.bytes().skip(mid).collect(),
        }
    }

    fn priority(&self) -> Option<u8> {
        let c = self.left.iter().find_map(|c| self.right.get(c))?.clone();
        if c <= 'Z' as u8 {
            Some(c - 'A' as u8 + 27)
        } else {
            Some(c - 'a' as u8 + 1)
        }
    }

    fn has(&self, c: u8) -> bool {
        self.left.contains(&c) || self.right.contains(&c)
    }

    fn iter(&self) -> impl Iterator<Item = u8> + '_ {
        self.left.iter().chain(self.right.iter()).copied()
    }
}

struct Group(Rucksack, Rucksack, Rucksack);

impl Group {
    fn new(input: &[&str]) -> Self {
        Self(
            Rucksack::new(input[0]),
            Rucksack::new(input[1]),
            Rucksack::new(input[2]),
        )
    }

    fn priority(&self) -> Option<u8> {
        let c = self
            .0
            .iter()
            .find_map(|c| (self.1.has(c) && self.2.has(c)).then(|| c))?;
        if c <= 'Z' as u8 {
            Some(c - 'A' as u8 + 27)
        } else {
            Some(c - 'a' as u8 + 1)
        }
    }
}

#[cfg(test)]
#[test]
fn test_priority() {
    let r = Rucksack::new("vJrwpWtwJgWrhcsFMMfFFhFp");
    assert_eq!(r.priority(), Some(16));
    let r = Rucksack::new("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL");
    assert_eq!(r.priority(), Some(38));
    let r = Rucksack::new("PmmdzqPrVvPwwTWBwg");
    assert_eq!(r.priority(), Some(42));
    let r = Rucksack::new("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn");
    assert_eq!(r.priority(), Some(22));
    let r = Rucksack::new("ttgJtRGJQctTZtZT");
    assert_eq!(r.priority(), Some(20));
    let r = Rucksack::new("CrZsJsPPZsGzwwsLwLmpwMDw");
    assert_eq!(r.priority(), Some(19));
}

#[test]
fn test_group_priority() {
    let g = Group::new(&[
        "vJrwpWtwJgWrhcsFMMfFFhFp",
        "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
        "PmmdzqPrVvPwwTWBwg",
    ]);
    assert_eq!(g.priority(), Some(18));
}
