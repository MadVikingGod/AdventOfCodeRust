use crate::util::Field;
use crate::util::Point;

pub fn read_input() -> Field<i64> {
    let input = include_str!("input.txt");
    parse_map(input)
}

pub fn parse_map(input: &str) -> Field<i64> {
    let mut map = Field::new();
    input.lines().enumerate().for_each(|(y, line)| {
        let y = y as i64;
        line.chars().enumerate().for_each(|(x, c)| {
            let x = x as i64;
            map.insert(Point { x, y }, c.to_digit(10).unwrap() as i64);
        });
    });
    map
}
