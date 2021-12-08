use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;

pub fn read_input() -> Vec<(Vec<&'static str>, Vec<&'static str>)> {
    let input = include_str!("input.txt");
    parse_input(input)
}

pub fn parse_input(input: &'static str) -> Vec<(Vec<&'static str>, Vec<&'static str>)> {
    input
        .lines()
        .map(|line| line.split_once('|').unwrap())
        .map(|(groups, display)| {
            (
                groups.split_whitespace().collect(),
                display.split_whitespace().collect(),
            )
        })
        .collect()
}

pub fn parse_groups(input: &[&str]) -> HashMap<String, u64> {
    let groups: Vec<HashSet<char>> = input
        .iter()
        .map(|&group| HashSet::from_iter(group.chars()))
        .collect();
    let mut map: HashMap<usize, HashSet<char>> = HashMap::new();
    for group in groups.iter() {
        let g = map
            .entry(group.len())
            .or_insert_with( || HashSet::from(['a', 'b', 'c', 'd', 'e', 'f', 'g']));
        *g = g.intersection(group).cloned().collect();
    }

    let seg_a: String = map[&3].difference(&map[&2]).cloned().collect::<String>();
    let seg_b: String = HashSet::from_iter(map[&6].difference(&map[&5]).cloned())
        .difference(&map[&2])
        .cloned()
        .collect::<String>();
    let seg_c: String = HashSet::from_iter(map[&4].difference(&map[&5]).cloned())
        .difference(&map[&6])
        .cloned()
        .collect::<String>();
    let seg_d: String = HashSet::from_iter(map[&4].difference(&map[&2]).cloned())
        .intersection(&map[&5])
        .cloned()
        .collect::<String>();
    let seg_e: String = HashSet::from_iter(map[&7].difference(&map[&6]).cloned())
        .difference(&map[&4])
        .cloned()
        .collect::<String>();
    let seg_f: String = HashSet::from_iter(map[&6].difference(&map[&5]).cloned())
        .intersection(&map[&2])
        .cloned()
        .collect::<String>();
    let seg_g: String = HashSet::from_iter(map[&5].difference(&map[&3]).cloned())
        .difference(&map[&4])
        .cloned()
        .collect::<String>();

    let zero = seg_a.clone() + &seg_b + &seg_c + &seg_e + &seg_f + &seg_g;
    let zero = zero.chars().sorted().collect::<String>();
    let one = seg_c.clone() + &seg_f;
    let one = one.chars().sorted().collect::<String>();
    let two = seg_a.clone() + &seg_c + &seg_d + &seg_e + &seg_g;
    let two = two.chars().sorted().collect::<String>();
    let three = seg_a.clone() + &seg_c + &seg_d + &seg_f + &seg_g;
    let three = three.chars().sorted().collect::<String>();
    let four = seg_b.clone() + &seg_c + &seg_d + &seg_f;
    let four = four.chars().sorted().collect::<String>();
    let five = seg_a.clone() + &seg_b + &seg_d + &seg_f + &seg_g;
    let five = five.chars().sorted().collect::<String>();
    let six = seg_a.clone() + &seg_b + &seg_d + &seg_e + &seg_f + &seg_g;
    let six = six.chars().sorted().collect::<String>();
    let seven = seg_a.clone() + &seg_c + &seg_f;
    let seven = seven.chars().sorted().collect::<String>();
    let eight = seg_a.clone() + &seg_b + &seg_c + &seg_d + &seg_e + &seg_f + &seg_g;
    let eight = eight.chars().sorted().collect::<String>();
    let nine = seg_a + &seg_b + &seg_c + &seg_d + &seg_f + &seg_g;
    let nine = nine.chars().sorted().collect::<String>();

    HashMap::from([
        (zero, 0),
        (one, 1),
        (two, 2),
        (three, 3),
        (four, 4),
        (five, 5),
        (six, 6),
        (seven, 7),
        (eight, 8),
        (nine, 9),
    ])
}
