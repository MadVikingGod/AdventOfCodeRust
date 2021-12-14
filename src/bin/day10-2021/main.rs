use advent_of_code::y2021::day10::*;
use itertools::Itertools;

fn main() {
    println!("Hello, world!");
    let input = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";
    let lines: Vec<Line> = input.lines().map(|line| Line{c: line.chars().collect()}).collect();
    let corrupt: u64 = lines.iter()
        .filter_map(|line| line.invalid())
        .map(|c| match c {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => 0
        })
        .sum();
    println!("{:?}", corrupt);
    let incompletes: Vec<_> = lines.iter().filter_map(|line| line.incomplete()).collect();
    let scores: Vec<_> = incompletes.iter()
        .map(|inc| {
            inc.chars().rev().fold(0, |acc, c| {
                let score = match c {
                    '(' => 1,
                    '[' => 2,
                    '{' => 3,
                    '<' => 4,
                    _ => 0
                };
                acc*5+score
            })})
        .sorted()
        .collect();
    println!("{:?}", scores[scores.len()/2]);


    let lines = read_input();
    let corrupt: u64 = lines.iter()
        .filter_map(|line| line.invalid())
        .map(|c| match c {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => 0
        })
        .sum();
    println!("{:?}", corrupt);
    let incompletes: Vec<_> = lines.iter().filter_map(|line| line.incomplete()).collect();
    let scores: Vec<_> = incompletes.iter()
        .map(|inc| {
            inc.chars().rev().fold(0_i64, |acc, c| {
                let score = match c {
                    '(' => 1,
                    '[' => 2,
                    '{' => 3,
                    '<' => 4,
                    _ => 0
                };
                acc*5+score
            })})
        .sorted()
        .collect();
    println!("{:?}", scores[scores.len()/2]);
}
