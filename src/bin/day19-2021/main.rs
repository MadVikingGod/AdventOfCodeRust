use std::{f64::consts, convert::TryFrom};

use advent_of_code::y2021::day19::*;
// Common tools
// use advent_of_code::util::Field;
// use advent_of_code::util::Point;
// use std::collections::HashMap;
// use std::collections::HashSet;
// use std::collections::VecDeque;
// use itertools::Itertools;


extern crate nalgebra as na;
use itertools::Itertools;
use na::{Vector3, Isometry3, Point3, Rotation3, Matrix3};

fn main() {
    println!("Hello, world!");
    // let pt00= Point3::new(-1.0,-1.0,1.0);
    let pt00= Point3::new(-1,-1,1);
    let v00= Vector3::new(-1,-1,1);
    let pt01: Point3<i32> = Point3::new(-2,-2,2);
    let pt02: Point3<i32> = Point3::new(-3,-3,3);
    let pt03: Point3<i32> = Point3::new(-2,-3,1);
    let pt04: Point3<i32> = Point3::new(5,6,-4);
    let pt05: Point3<i32> = Point3::new(8,0,7);

    // let pt10 = Point3::new(1.0,-1.0,1.0);
    let pt10 = Point3::new(1,-1,1);
    let v10 = Vector3::new(1,-1,1);
    let pt11: Point3<i32> = Point3::new(2,-2,2);
    let pt12: Point3<i32> = Point3::new(3,-3,3);
    let pt13: Point3<i32> = Point3::new(2,-1,3);
    let pt14: Point3<i32> = Point3::new(-5,4,-6);
    let pt15: Point3<i32> = Point3::new(-8,-7,0);

    
    let pt = v00*v10.transpose();
    let x = Matrix3::from_row_slice(&[1,0,0,0,0,-1,0,1,0]);
    println!("{}", x);

    for x in get_rot_matrixes() {
        println!("{}", x*v10);
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
