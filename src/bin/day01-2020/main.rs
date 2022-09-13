use advent_of_code::y2020::day01::read_input;
use advent_of_code::y2020::day01::find_pair;
use advent_of_code::y2020::day01::find_tripple;

fn main() {
    println!("Hello, world!");
    let set = read_input();
    
    println!("{:?}",find_pair(2020, &set).unwrap());

    println!("{:?}", find_tripple(&set).unwrap());
}

