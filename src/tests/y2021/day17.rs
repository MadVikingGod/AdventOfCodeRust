use std::collections::HashSet;

use crate::util::Point;
use crate::y2021::day17::*;

#[test]
fn test_nothing() -> Result<(), String> {
    Ok(())
}

#[test]
fn test_x_steps() -> Result<(), String> {
    let hm = x_steps(20, 30);

    println!("{:?}", hm);
    let vel = get_points(hm, -10, -5);
    fmt_points(&vel);
    assert_eq!(vel.len(), 112);

    // Err(format!("{:?}", vel))

    Ok(())
    // Err(format!("{:?}", hm))
}

fn fmt_points(vel: &HashSet<Point>) {
    vel.iter().for_each(|pt| print!("{},{} ", pt.x, pt.y));
    println!("");
}
