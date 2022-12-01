// Common tools
// use crate::util::Field;
use crate::util::Point;
use std::collections::HashMap;
use std::collections::HashSet;
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

const steps_max: i64 = 400;

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
        let stop = (vx + 1) * vx / 2;
        if stop >= x_min && stop <= x_max {
            for s in vx..steps_max {
                out.entry(s).or_insert(Vec::new()).push(vx);
            }
        }
    }
    out
}

pub fn get_points(steps: HashMap<i64, Vec<i64>>, y_min: i64, y_max: i64) -> HashSet<Point> {
    let mut velocities: HashSet<Point> = HashSet::new();

    // Need to deal with when X stops
    for vy in y_min.. {
        let mut y = 0;
        for s in 1..steps_max + 1 {
            y += vy - (s - 1);
            if y < y_min {
                break;
            }
            if y <= y_max {
                if let Some(vels) = steps.get(&s) {
                    vels.iter().for_each(|vx| {
                        velocities.insert((vx, &vy).into());
                    });
                }
            }

            if s == steps_max && y > y_max {
                return velocities;
            }
        }
    }
    velocities
}
