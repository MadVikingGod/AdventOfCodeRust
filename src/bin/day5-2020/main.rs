use advent_of_code::y2020::day5::read_input;

fn main() {
    let mut seat_nums = read_input();

    println!("{:?}", seat_nums.iter().max().unwrap());
    println!("{:?}", seat_nums.iter().min().unwrap());

    let max = seat_nums.iter().max().unwrap();
    let min = seat_nums.iter().min().unwrap();
    let sum = seat_nums.iter().sum::<u64>();

    println!("{:?}", nat_sum(*max) - nat_sum(*min - 1) - sum);

    seat_nums.sort_unstable();
    // println!("{:?}",seat_nums)
}

fn nat_sum(n: u64) -> u64 {
    n * (n + 1) / 2
}
