use advent_of_code::y2020::day03::{read_input, Slope};


fn main() {
    println!("Hello, world!");
    let map = read_input();
    println!("{:?}", map.slope(3,1).filter(|x| *x).count());
    let slopes: Vec<Slope> = vec![map.slope(1, 1), map.slope(3,1), map.slope(5,1), map.slope(7, 1), map.slope(1, 2)];
    println!("{:?}", slopes.iter().map(|s| s.clone().filter(|x| *x).count()).product::<usize>());
}