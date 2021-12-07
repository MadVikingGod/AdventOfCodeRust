use std::collections::HashSet;

pub fn read_input() -> HashSet<i64> {
    let bytes = include_str!("input.txt");
    bytes.lines().map(|x| x.parse().unwrap()).collect()
}

pub fn find_pair(target: i64, set: &HashSet<i64>) -> Option<i64> {
    set.iter()
        .filter_map(|x| set.get(&(target - *x)).map(|y| x * y))
        .next()
}

pub fn find_tripple(set: &HashSet<i64>) -> Option<i64> {
    set.iter()
        .filter_map(|i| find_pair(2020 - i, set).map(|product| i * product))
        .next()
}
