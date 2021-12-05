pub mod board;

pub fn read_input() -> (Vec<i64>, Vec<board::Board>) {
    let input = include_str!("input.txt");
    let mut input = input.split("\n\n");
    let numbers: Vec<i64> = input.next().unwrap().split(",").filter_map(|val| val.parse().ok()).collect();
    let boards = parse_boards(input.collect());
    (numbers, boards)
}

pub fn parse_boards(list: Vec<&str>) -> Vec<board::Board> {
    list.iter().map(|b|
        board::from_list(b.lines().collect()).unwrap()).collect()
}