use advent_of_code::y2020::day1::read_input;
use advent_of_code::y2020::day1::find_pair;
use advent_of_code::y2020::day1::find_tripple;

fn main() {
    println!("Hello, world!");
    let set = read_input();
    let (i,j) = find_pair(2020, &set).unwrap();
    println!("{:?}",i*j);

    let (i,j,k) = find_tripple(&set).unwrap();
    println!("{:?}", i*j*k)
}

