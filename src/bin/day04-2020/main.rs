use advent_of_code::y2020::day04::read_input;

fn main() {
    let passports = read_input();
    println!("{:?}", passports.iter().filter(|p| p.is_valid()).count());
    println!(
        "{:?}",
        passports.iter().filter(|p| p.is_valid_extended()).count()
    );
}
