// use nalgebra::Dim;
// Common tools
// use crate::util::Field;
// use crate::util::Point;
// use std::collections::HashMap;
// use std::collections::HashSet;
// use std::collections::VecDeque;
// use itertools::Itertools;
use regex::Regex;

pub fn read_input() -> Vec<Instruction> {
    let input = include_str!("input.txt");
    input.lines().map(|l| l.into()).collect()
}

#[derive(Debug, PartialEq)]
pub struct Instruction {
    kind: Kind,
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
}

pub type Lights = [[bool; 1000]; 1000];
pub type DimLights = [[i32; 1000]; 1000];

#[derive(Debug, PartialEq)]
pub enum Kind {
    TurnOff,
    TurnOn,
    Toggle,
    Unknown,
}

impl From<&str> for Instruction {
    fn from(s: &str) -> Self {
        let re: Regex =
            Regex::new(r"(?P<kind>.*) (?P<x1>\d+),(?P<y1>\d+) through (?P<x2>\d+),(?P<y2>\d+)")
                .unwrap();

        let caps = re.captures(s).unwrap();
        let kind = match &caps["kind"] {
            "toggle" => Kind::Toggle,
            "turn on" => Kind::TurnOn,
            "turn off" => Kind::TurnOff,
            _ => Kind::Unknown,
        };

        Instruction {
            kind: kind,
            x1: caps["x1"].parse().unwrap(),
            y1: caps["y1"].parse().unwrap(),
            x2: caps["x2"].parse().unwrap(),
            y2: caps["y2"].parse().unwrap(),
        }
    }
}

impl Instruction {
    pub fn apply(&self, lights: &mut Lights) {
        for y in self.y1..self.y2+1 {
            for x in self.x1..self.x2+1 {
                lights[y][x] = match self.kind {
                    Kind::Toggle => lights[y][x] ^ true,
                    Kind::TurnOff => false,
                    Kind::TurnOn => true,
                    _ => false,
                };
            }
        }
    }
    pub fn applyDim(&self, lights: &mut DimLights) {
        for y in self.y1..self.y2+1 {
            for x in self.x1..self.x2+1 {
                lights[y][x] += match self.kind {
                    Kind::Toggle => 2,
                    Kind::TurnOff => -1,
                    Kind::TurnOn => 1,
                    _ => 0,
                };
                if lights[y][x] <0 {
                    lights[y][x] = 0;
                }
            }
        }
    }
}

pub fn count(lights: &Lights) -> usize {
    lights
        .iter()
        .flatten()
        .filter(|l| **l)
        .count()
}

pub fn count_dim(lights: &DimLights) -> i64 {
    lights.iter().flatten().fold(0, |acc, &l| acc + l as i64)
}

#[cfg(test)]
#[test]
fn test_instruction_parse() -> Result<(), String> {
    let input = "turn on 0,0 through 999,999";
    let want = Instruction {
        kind: Kind::TurnOn,
        x1: 0,
        y1: 0,
        x2: 999,
        y2: 999,
    };
    assert_eq!(Instruction::from(input), want);

    Ok(())
}

#[test]
fn test_apply() -> Result<(),String> {
    let inst = Instruction::from("turn on 0,0 through 50,0");
    let mut lights: Lights = [[false; 1000]; 1000];
    assert_eq!(0,count(&lights));
    inst.apply(&mut lights);
    assert_eq!(51, count(&lights));

    Ok(())
}
