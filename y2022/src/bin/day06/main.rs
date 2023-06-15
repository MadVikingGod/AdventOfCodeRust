use std::collections::HashSet;

fn main() {
    let input: Vec<char> = include_str!("input.txt").chars().collect();
    let mut buffer = Buffer::new(4);
    let p = input.iter().position(|&c| {
        buffer.insert(c);
        !buffer.has_repeat()
    }).unwrap();
    println!("part1: {}", p+1);


    let mut buffer = Buffer::new(14);
    let p = input.iter().position(|&c| {
        buffer.insert(c);
        !buffer.has_repeat()
    }).unwrap();
    println!("part2: {}", p+1);
}

struct Buffer {
    len : usize,
    data : Vec<char>,
}

impl Buffer {
    fn new(size: usize) -> Self {
        Self {
            len: size,
            data: Vec::with_capacity(size),
        }
    }

    fn has_repeat(&self) -> bool {
        if self.data.len() < self.len {
            return true
        }
        let mut coll: HashSet<char> =HashSet::new();
        for c in self.data.iter() {
            if coll.contains(c) {
                return true
            }
            coll.insert(*c);
        }
        false
    }
    fn insert(&mut self, c: char) {
        if self.data.len() == self.len {
            self.data.remove(0);
        }
        self.data.push(c);
    }
}

#[cfg(test)]
#[test]
fn test_() {
    let input: Vec<char> = "bvwbjplbgvbhsrlpgdmjqwftvncz".chars().collect();
    let mut buffer = Buffer::new(4);
    let p = input.iter().position(|&c| {
        buffer.insert(c);
        !buffer.has_repeat()
    }).unwrap();
    assert_eq!(p, 4);

    let input: Vec<char> = "nppdvjthqldpwncqszvftbrmjlhg".chars().collect();
    let mut buffer = Buffer::new(4);
    let p = input.iter().position(|&c| {
        buffer.insert(c);
        !buffer.has_repeat()
    }).unwrap();
    assert_eq!(p, 5);

    let input: Vec<char> = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".chars().collect();
    let mut buffer = Buffer::new(4);
    let p = input.iter().position(|&c| {
        buffer.insert(c);
        !buffer.has_repeat()
    }).unwrap();
    assert_eq!(p, 9);

    let input: Vec<char> = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".chars().collect();
    let mut buffer = Buffer::new(4);
    let p = input.iter().position(|&c| {
        buffer.insert(c);
        !buffer.has_repeat()
    }).unwrap();
    assert_eq!(p, 10);
}
