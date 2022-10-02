use advent_of_code::y2021::day08::parse_groups;
use advent_of_code::y2021::day08::parse_input;
use advent_of_code::y2021::day08::read_input;

use itertools::Itertools;

fn main() {
    println!("Hello, world!");
    let example_input =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
    edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
    fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
    fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
    aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
    fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
    dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
    bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
    egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
    gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";
    let lines = parse_input(example_input);
    let digits = lines.iter().map(|(_, digits)| digits);
    println!(
        "{}",
        digits
            .flatten()
            .filter(|disp| disp.len() == 2 || disp.len() == 3 || disp.len() == 4 || disp.len() == 7)
            .count()
    );

    let lines = read_input();
    let digits = lines.iter().map(|(_, digits)| digits);
    println!(
        "{}",
        digits
            .flatten()
            .filter(|disp| disp.len() == 2 || disp.len() == 3 || disp.len() == 4 || disp.len() == 7)
            .count()
    );

    let lines = parse_input(example_input);
    let displays: u64 = lines
        .iter()
        .map(|(groups, digits)| {
            let map = parse_groups(groups);
            digits
                .iter()
                .map(|&d| map[&d.chars().sorted().collect::<String>()])
                .fold(0, |acc, d| acc * 10 + d)
        })
        .sum();
    println!("{:?}", displays);

    let lines = read_input();
    let displays: u64 = lines
        .iter()
        .map(|(groups, digits)| {
            let map = parse_groups(groups);
            digits
                .iter()
                .map(|&d| map[&d.chars().sorted().collect::<String>()])
                .fold(0, |acc, d| acc * 10 + d)
        })
        .sum();
    println!("{:?}", displays);
}
