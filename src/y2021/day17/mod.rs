// Common tools
// use crate::util::Field;
// use crate::util::Point;
use std::collections::HashMap;
// use std::collections::HashSet;
// use std::collections::VecDeque;
// use itertools::Itertools;

//target area: x=153..199, y=-114..-75
pub fn read_input() -> Vec<&'static str> {
    let input = include_str!("input.txt");
    input.lines().collect()
}

// xf = s/2*(2*vx - (s-1))
// minvx 153>=s*(2*vx-(s-1))/2 =  vx=17
// maxvx 199
// const vx_min: i64 = 17;
// const vx_max: i64 = 199;
// const x_min: i64 = 153;
// const x_max: i64 = 199;

pub fn x_steps(x_min: i64, x_max: i64) -> HashMap<i64, Vec<i64>> {
    let mut vx_min = 0;
    for i in 1.. {
        if (i + 1) * i / 2 >= x_min {
            vx_min = i;
            break;
        };
    }

    let mut out: HashMap<i64, Vec<i64>> = HashMap::new();
    for vx in vx_min..(x_max + 1) {
        let mut x = 0;
        for s in 1..vx {
            x = x + vx - s + 1;
            if x > x_max {
                break;
            };
            if x >= x_min {
                out.entry(s).or_insert(Vec::new()).push(vx);
            }
        }
    }
    out
}
