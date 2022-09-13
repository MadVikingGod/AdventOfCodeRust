use advent_of_code::y2021::day03::read_input;
use advent_of_code::y2021::day03::calculat_gama;
use advent_of_code::y2021::day03::{calculate_oxygen, calculate_co2};
fn main() {
    println!("Hello, world!");
    let input = read_input();
    println!("{:?}", calculat_gama(input.clone()));
    println!("{:?}", calculate_oxygen(input.clone()));
    println!("{:?}", calculate_co2(input));
}
//011100101100   1836
//100011010011   2259

//010110010011   1427
//100111000110   2502