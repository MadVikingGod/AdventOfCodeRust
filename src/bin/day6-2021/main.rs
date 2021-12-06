use advent_of_code::y2021::day6::read_input;

fn main() {
    println!("Hello, world!");
    let school = read_input();
    println!("{}", school.clone().skip(79).next().unwrap().count_fish());
    println!("{}", school.skip(255).next().unwrap().count_fish());
}
