// initial state
/*
                    [Q]     [P] [P]
                [G] [V] [S] [Z] [F]
            [W] [V] [F] [Z] [W] [Q]
        [V] [T] [N] [J] [W] [B] [W]
    [Z] [L] [V] [B] [C] [R] [N] [M]
[C] [W] [R] [H] [H] [P] [T] [M] [B]
[Q] [Q] [M] [Z] [Z] [N] [G] [G] [J]
[B] [R] [B] [C] [D] [H] [D] [C] [N]
 1   2   3   4   5   6   7   8   9
*/

use std::fmt;

fn main() {
    let mut stacks = Stacks::new();
    include_str!("input.txt")
        .lines()
        .map(parse_line)
        .for_each(|(count, from, to)| {
            stacks.move_count(count, from, to);
        });
    println!("Part 1: {:?}", stacks);

    let mut stacks = Stacks::new();
    include_str!("input.txt")
        .lines()
        .map(parse_line)
        .for_each(|(count, from, to)| {
            stacks.move2(count, from, to);
        });
    println!("Part 2: {:?}", stacks);
}

fn parse_line(line: &str) -> (usize, usize, usize) {
    let parts: Vec<&str> = line.split_whitespace().collect();

    let count = parts[1].parse::<usize>().unwrap();
    let from = parts[3].parse::<usize>().unwrap();
    let to = parts[5].parse::<usize>().unwrap();
    (count, from, to)
}

struct Stacks {
    stacks: Vec<Vec<char>>,
}
impl Stacks {
    fn new() -> Self {
        Self {
            stacks: vec![
                vec![],                                       //0
                vec!['B', 'Q', 'C'],                          //1
                vec!['R', 'Q', 'W', 'Z'],                     //2
                vec!['B', 'M', 'R', 'L', 'V'],                //3
                vec!['C', 'Z', 'H', 'V', 'T', 'W'],           //4
                vec!['D', 'Z', 'H', 'B', 'N', 'V', 'G'],      //5
                vec!['H', 'N', 'P', 'C', 'J', 'F', 'V', 'Q'], //6
                vec!['D', 'G', 'T', 'R', 'W', 'Z', 'S'],      //7
                vec!['C', 'G', 'M', 'N', 'B', 'W', 'Z', 'P'], //8
                vec!['N', 'J', 'B', 'M', 'W', 'Q', 'F', 'P'], //9
            ],
        }
    }
    fn new_test() -> Self {
        Self {
            stacks: vec![
                vec![],              //0
                vec!['Z', 'N'],      //0
                vec!['M', 'C', 'D'], //0
                vec!['P'],           //0
            ],
        }
    }

    fn move_count(&mut self, count: usize, from: usize, to: usize) {
        let tos = &mut self.stacks[to].to_owned();
        let froms = &mut self.stacks[from].to_owned();

        for _ in 0..count {
            tos.push(froms.pop().unwrap());
        }
        self.stacks[from] = froms.to_owned();
        self.stacks[to] = tos.to_owned();
    }

    fn move2(&mut self, count: usize, from: usize, to: usize) {
        let tos = &mut self.stacks[to].to_owned();
        let froms = &mut self.stacks[from].to_owned();
        let mid = froms.len() - count;
        for _ in 0..count {
            tos.append(&mut froms.split_off(mid))
        }
        self.stacks[from] = froms.to_owned();
        self.stacks[to] = tos.to_owned();
    }
    fn get_tops(&self) -> Vec<char> {
        self.stacks
            .iter()
            .skip(1)
            .filter_map(|s| Some(s.last()?.to_owned()))
            .collect()
    }
}

impl fmt::Debug for Stacks {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.get_tops().iter().collect::<String>().as_str())
    }
}

#[cfg(test)]
#[test]
fn test_part1() {
    let mut stacks = Stacks::new_test();
    stacks.move_count(1, 2, 1);
    assert_eq!(stacks.stacks[1], vec!['Z', 'N', 'D']);
    assert_eq!(stacks.stacks[2], vec!['M', 'C']);

    stacks.move_count(3, 1, 3);
    assert_eq!(stacks.stacks[1], vec![]);
    assert_eq!(stacks.stacks[3], vec!['P', 'D', 'N', 'Z']);

    stacks.move_count(2, 2, 1);
    stacks.move_count(1, 1, 2);

    assert_eq!(stacks.get_tops(), vec!['C', 'M', 'Z']);
}

#[test]
fn test_part2() {
    let mut stacks = Stacks::new_test();
    stacks.move2(1, 2, 1);
    assert_eq!(stacks.stacks[1], vec!['Z', 'N', 'D']);
    assert_eq!(stacks.stacks[2], vec!['M', 'C']);
    stacks.move2(3, 1, 3);
    assert_eq!(stacks.stacks[1], vec![]);
    assert_eq!(stacks.stacks[3], vec!['P', 'Z', 'N', 'D']);

    stacks.move2(2, 2, 1);
    stacks.move2(1, 1, 2);

    assert_eq!(stacks.get_tops(), vec!['M', 'C', 'D'])
}
