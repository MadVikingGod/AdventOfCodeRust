use advent_of_code::y2021::day01::read_input;
use advent_of_code::y2021::day01::count_increase;
use advent_of_code::y2021::day01::count_window_increase;

fn main() {
    println!("Hello, world!");
    let input = read_input();
    println!("{}", count_increase(&input));
    println!("{}", count_window_increase(&input, 3));
}