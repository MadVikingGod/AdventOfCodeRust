// Common tools
// use crate::util::Field;
// use crate::util::Point;
// use std::collections::HashMap;
// use std::collections::HashSet;
// use std::collections::VecDeque;
// use itertools::Itertools;

use std::ops::Mul;

use itertools::Itertools;
use nalgebra::{Vector3, Matrix3};


pub fn read_input() -> Vec<Scanner> {
    let input = include_str!("input.txt");
    input.split("\n\n").map(Scanner::from_string).collect_vec()
}

type Coord = Vector3<i32>;
pub struct Scanner {
    rel_coords: Option<Coord>,
    readings: Vec<Coord>
}


impl Scanner {
    pub fn from_string(s: &str) -> Self {
        let readings = s.lines().skip(1)
            .map(|line| {
                let mut spl = line.split(',');
                Vector3::from_column_slice(&[spl.next().unwrap().parse().unwrap(), spl.next().unwrap().parse().unwrap(), spl.next().unwrap().parse().unwrap()])        
            })
            .collect_vec();
        Self{rel_coords: None, readings: readings}
    }
}

impl Mul<Matrix3<i32>> for Scanner {
    type Output = Self;
    fn mul(self, rhs: Matrix3<i32>) -> Self::Output {
        let readings = self.readings.iter()
            .map(|v| rhs*v)
            .collect_vec();
        Self{rel_coords: self.rel_coords.clone(), readings}
    }
}

fn get_rot_matrixes() -> Vec<Matrix3<i32>> {
    let rots = vec!["I", "X", "Y", "XX", "XY", "YX", "YY", "XXX", "XXY", "XYX", "XYY", "YXX", "YYX", "YYY", "XXXY", "XXYX", "XXYY", "XYXX", "XYYY", "YXXX", "YYYX", "XXXYX", "XYXXX", "XYYYX"];
    let i = Matrix3::identity();
    let x = Matrix3::from_row_slice(&[1,0,0,0,0,-1,0,1,0]);
    let y = Matrix3::from_row_slice(&[0, 0, 1, 0, 1, 0, -1, 0, 0]);

    rots.into_iter().map(|s| s.chars().fold(i.clone(), |acc, ch| {
        let mat = match ch {
            'I' => &i,
            'X' => &x,
            'Y' => &y,
            _ => unreachable!()
        };
        acc * mat
    })).collect_vec()
}