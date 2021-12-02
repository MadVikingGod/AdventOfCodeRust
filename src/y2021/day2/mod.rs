use std::str::FromStr;

pub fn read_input() -> Vec<Instruction> {
    let bytes = include_str!("input.txt");
    bytes.lines().map(|x| x.parse().unwrap()).collect()
}

#[derive(Debug, Clone, PartialEq)]
pub enum Direction {
    Forward,
    Down,
    Up,
}

impl FromStr for Direction {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "forward" => Ok(Self::Forward),
            "down" => Ok(Self::Down),
            "up" => Ok(Self::Up),
            _ => Err("direction undefined".to_string())
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Instruction {
    pub dir: Direction,
    pub size: i64,
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, size) = match s.split_once(' ') {
            Some((d,s)) => (d,s),
            None => return Err("did not match format 'dir 1'".to_string()),
        };
        Ok(Instruction {
            dir: dir.parse().unwrap(),
            size: size.parse().unwrap(),
        })
    }
}

pub fn calculate_position(list: Vec<Instruction>) -> (i64,i64) {
    list.iter().fold((0,0), |acc, ins| {
        match ins.dir {
            Direction::Forward => (acc.0, acc.1+ins.size),
            Direction::Down => (acc.0 + ins.size, acc.1),
            Direction::Up => (acc.0 - ins.size, acc.1),
        }
    })
}

pub fn calculate_advance_position(list: Vec<Instruction>) -> (i64,i64) {
    let (depth,horiz,_) = list.iter().fold((0,0,0), |acc, ins| {
        match ins.dir {
            Direction::Forward => (acc.0 + ins.size*acc.2, acc.1+ins.size, acc.2),
            Direction::Down => (acc.0, acc.1, acc.2+ins.size),
            Direction::Up => (acc.0, acc.1, acc.2-ins.size),
        }
    });
    (depth, horiz)
}