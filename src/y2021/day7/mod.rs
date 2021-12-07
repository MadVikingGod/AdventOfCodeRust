pub fn read_input() -> Vec<i64> {
    let input = include_str!("input.txt");
    input
        .split(",")
        .map(|val| val.parse::<i64>().unwrap())
        .collect()
}

pub fn distance(x: i64, y: i64) -> i64 {
    (x - y).abs()
}

pub fn group_distance(list: &Vec<i64>, pos: i64) -> i64 {
    list.iter().map(|&x| distance(x, pos)).sum()
}

pub fn find_min_distance(list: &Vec<i64>) -> (i64, i64) {
    let &min = list.iter().min().unwrap();
    let &max = list.iter().max().unwrap();

    let mut pos = 0;
    let mut value = i64::MAX;
    for i in min..max {
        let dist = group_distance(&list, i);
        if dist < value {
            pos = i;
            value = dist;
        };
    }
    (pos, value)
}

pub fn advance_distance(x: i64, y: i64) -> i64 {
    match distance(x, y) {
        0 => 0,
        z => (1..z + 1).sum(),
    }
}

pub fn advance_group_distance(list: &Vec<i64>, pos: i64) -> i64 {
    list.iter().map(|&x| advance_distance(x, pos)).sum()
}

pub fn find_min_advance_distance(list: &Vec<i64>) -> (i64, i64) {
    let &min = list.iter().min().unwrap();
    let &max = list.iter().max().unwrap();

    let mut pos = 0;
    let mut value = i64::MAX;
    for i in min..max {
        let dist = advance_group_distance(&list, i);
        if dist < value {
            pos = i;
            value = dist;
        };
    }
    (pos, value)
}
