use advent_of_code::y2021::day6::read_input;

fn main() {
    println!("Hello, world!");
    let mut school = read_input();
    println!("{}", school.clone().nth(79).unwrap().count_fish());
    println!("{}", school.nth(255).unwrap().count_fish());
}
