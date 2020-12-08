use advent_of_code::y2020::day6::{read_input, read_input2};

fn main() {
    let groups = read_input();
    let group_sizes = groups.iter().map(|g| g.len()).collect::<Vec<usize>>();
    println!("{:?}", group_sizes.iter().sum::<usize>());

    let groups = read_input2();
    let group_sizes = groups.iter().map(|g| g.len()).collect::<Vec<usize>>();
    println!("{:?}", group_sizes.iter().sum::<usize>());
}
