use advent_of_code::y2021::day02::read_input;
use advent_of_code::y2021::day02::calculate_position;
use advent_of_code::y2021::day02::calculate_advance_position;

fn main() {
    let input = read_input();
    let (depth,horiz) =  calculate_position(input.clone());
    println!("{:?}", depth*horiz);
    let (depth2,horiz2) =  calculate_advance_position(input);
    println!("{:?}", depth2*horiz2);
}