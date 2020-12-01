use std::collections::HashSet;

pub fn read_input() -> HashSet<i64> {
    let bytes = include_str!("input.txt");
    bytes.lines().map(|x| x.parse().unwrap()).collect()
}

pub fn find_pair(target: i64, set: &HashSet<i64>) -> Option<(i64, i64)> {
    for i in set.iter() {
       if set.contains(&(target-i)) {
           return Some((*i,target-i))
        };
    };
    None
}

pub fn find_tripple(set: &HashSet<i64>) -> Option<(i64,i64,i64)> {
    for i in set.iter() {
        match find_pair(2020-i, set) {
            Some(pair) => return Some((*i, pair.0, pair.1)),
            None => (),
        }
    }
    None
}