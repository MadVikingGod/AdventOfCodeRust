fn main() {
    let input = include_str!("input.txt");
    let rounds = parse_rounds(input);
    let part1 = rounds.iter().map(|round| round.score()).sum::<u64>();
    println!("Part 1: {}", part1);

    let part2 = rounds.iter().map(|round| round.score2()).sum::<u64>();
    println!("Part 2: {}", part2);
}

fn parse_rounds(input: &str) -> Vec<Round> {
    input.lines().map(|line| Round::new(line.trim())).collect()
}

#[derive(Debug, PartialEq)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}
enum WLD {
    Win,
    Lose,
    Draw,
}

impl From<RPS> for WLD {
    fn from(rps: RPS) -> Self {
        match rps {
            RPS::Rock => WLD::Lose,
            RPS::Paper => WLD::Draw,
            RPS::Scissors => WLD::Win,
        }
    }
}
impl From<&RPS> for WLD {
    fn from(rps: &RPS) -> Self {
        match rps {
            RPS::Rock => WLD::Lose,
            RPS::Paper => WLD::Draw,
            RPS::Scissors => WLD::Win,
        }
    }
}

#[derive(Debug)]
struct Round {
    elf: RPS,
    me: RPS,
}

impl Round {
    fn new(input: &str) -> Self {
        Self {
            elf: parse_elf(&input[..1]),
            me: parse_me(&input[2..3]),
        }
    }
    fn score(&self) -> u64 {
        let shape = match self.me {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3,
        };
        let outcome = if &self.elf == &self.me {
            3
        } else if match (&self.elf, &self.me) {
            (RPS::Rock, RPS::Paper) => true,
            (RPS::Paper, RPS::Scissors) => true,
            (RPS::Scissors, RPS::Rock) => true,
            _ => false,
        } {
            6
        } else {
            0
        };

        shape + outcome
    }
    fn score2(&self) -> u64 {
        let wld: WLD = (&self.me).into();
        let outcome = match wld {
            WLD::Win => 6,
            WLD::Lose => 0,
            WLD::Draw => 3,
        };
        let shape = match (wld, &self.elf) {
            (WLD::Win, RPS::Rock) => 2,      // Paper
            (WLD::Win, RPS::Paper) => 3,     // Scissors
            (WLD::Win, RPS::Scissors) => 1,  // Rock
            (WLD::Lose, RPS::Rock) => 3,     // Scissors
            (WLD::Lose, RPS::Paper) => 1,    // Rock
            (WLD::Lose, RPS::Scissors) => 2, // Paper
            (WLD::Draw, RPS::Rock) => 1,     // Rock
            (WLD::Draw, RPS::Paper) => 2,    // Paper
            (WLD::Draw, RPS::Scissors) => 3, // Scissors
        };
        shape + outcome
    }
}
fn parse_elf(input: &str) -> RPS {
    match input {
        "A" => RPS::Rock,
        "B" => RPS::Paper,
        "C" => RPS::Scissors,
        _ => panic!("Invalid input"),
    }
}
fn parse_me(input: &str) -> RPS {
    match input {
        "X" => RPS::Rock,
        "Y" => RPS::Paper,
        "Z" => RPS::Scissors,
        _ => panic!("Invalid input"),
    }
}

#[cfg(test)]
#[test]
fn test_round_new() {
    let input = "A Y";
    let round = Round::new(input);
    assert_eq!(round.elf, RPS::Rock);
    assert_eq!(round.me, RPS::Paper);

    assert_eq!(round.score(), 8);
    assert_eq!(round.score2(), 4);
}
#[test]
fn test_score() {
    let input = r"A Y
B X
C Z";
    let score = parse_rounds(input)
        .iter()
        .map(|round| round.score())
        .sum::<u64>();
    let score2 = parse_rounds(input)
        .iter()
        .map(|round| round.score2())
        .sum::<u64>();
    assert_eq!(score, 15);
    assert_eq!(score2, 12);
}
