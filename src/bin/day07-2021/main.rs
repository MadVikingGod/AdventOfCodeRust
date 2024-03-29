use advent_of_code::y2021::day07::find_min_advance_distance;
use advent_of_code::y2021::day07::find_min_distance;
use advent_of_code::y2021::day07::group_distance;
use advent_of_code::y2021::day07::read_input;

fn main() {
    println!("Hello, world!");

    println!("{}", group_distance(&[16, 1, 2, 0, 4, 2, 7, 1, 2, 14], 2));
    println!("{}", group_distance(&[16, 1, 2, 0, 4, 2, 7, 1, 2, 14], 4));
    println!("{}", group_distance(&[16, 1, 2, 0, 4, 2, 7, 1, 2, 14], 5));

    println!();

    let input = read_input();
    println!("{:?}", find_min_distance(&input));
    println!("{:?}", find_min_advance_distance(&input));
}
