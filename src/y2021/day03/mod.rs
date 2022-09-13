pub fn read_input() -> Vec<&'static str> {
    let input = include_str!("input.txt");
    input.lines().collect()
}

fn calculate_score(list: &[&str]) -> Vec<usize> {
    let mut acc: Vec<usize> = vec![0; list[0].len()];
    list.iter().for_each(|line| {
        line.chars().enumerate().for_each(|(i, c)| {
            if c == '1' {
                acc[i] += 1
            }
        });
    });
    acc
}

pub fn calculat_gama(list: Vec<&str>) -> String {
    let acc = calculate_score(&list);
    acc.iter()
        .map(|digit| if digit > &(&list.len() / 2) { "1" } else { "0" })
        .collect::<Vec<&str>>()
        .join("")
}

pub fn calculate_epsilon(gamma: String) -> String {
    gamma
        .chars()
        .map(|c| match c {
            '1' => "0",
            _ => "1",
        })
        .collect::<Vec<&str>>()
        .join("")
}

fn filter_oxygen(list: Vec<&str>, pos: usize) -> Vec<&str> {
    let score = calculate_score(&list);
    // println!("POS {} score {:?}",pos,score);
    let filter = if score[pos] >= (list.len() - score[pos]) {
        '1'
    } else {
        '0'
    };
    // println!("FILTER {}", filter);
    list.iter()
        .filter_map(|&line| {
            if line.chars().nth(pos)? == filter {
                Some(line)
            } else {
                None
            }
        })
        .collect()
}

fn filter_co2(list: Vec<&str>, pos: usize) -> Vec<&str> {
    let score = calculate_score(&list);
    // println!("POS {} score {:?} ",pos,score);
    let filter = if score[pos] < (list.len() - score[pos]) {
        '1'
    } else {
        '0'
    };
    // println!("FILTER {}", filter);
    list.iter()
        .filter_map(|&line| {
            if line.chars().nth(pos)? == filter {
                Some(line)
            } else {
                None
            }
        })
        .collect()
}

pub fn calculate_oxygen(list: Vec<&str>) -> String {
    let mut acc = list.clone();
    for i in 0..list[0].len() {
        // println!("RUN {} ####\n{:?}", i, acc);
        if acc.len() == 1 {
            return acc[0].to_string();
        }
        acc = filter_oxygen(acc, i)
    }
    acc[0].to_string()
}

pub fn calculate_co2(list: Vec<&str>) -> String {
    let mut acc = list.clone();
    for i in 0..list[0].len() {
        // println!("RUN {} ####\n{:?}", i, acc);
        if acc.len() == 1 {
            return acc[0].to_string();
        }
        acc = filter_co2(acc, i)
    }
    acc[0].to_string()
}
