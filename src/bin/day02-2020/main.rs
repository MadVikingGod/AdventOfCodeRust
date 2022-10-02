use advent_of_code::y2020::day02::read_input;

fn main() {
    println!("Hello, world!");
    let passwords = read_input();
    println!("{}", passwords.len());
    println!(
        "{:?}",
        passwords
            .iter()
            .filter(|t| t.0.check_password(t.1.clone()))
            .count()
    );

    println!(
        "{:?}",
        passwords
            .iter()
            .filter(|t| t.0.check_toboggan_password(t.1.clone()))
            .count()
    );
}
