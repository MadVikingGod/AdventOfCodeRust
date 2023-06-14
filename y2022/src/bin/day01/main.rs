fn main() {
    let mut elfs = read_input();
    println!("Part 1: {}", elfs.iter().max().unwrap());
    elfs.sort();
    println!("Part 2: {}", elfs.iter().rev().take(3).sum::<i64>());
}

pub fn read_input() -> Vec<i64> {
    let input = include_str!("input.txt");
    parse_elfs(input)
}

pub fn parse_elfs(input: &str) -> Vec<i64> {
    input
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .map(|line| line.parse::<i64>().unwrap())
                .sum::<i64>()
        })
        .collect()
}
