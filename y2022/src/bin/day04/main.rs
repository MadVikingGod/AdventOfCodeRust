fn main() {
    let pairs = include_str!("input.txt")
        .lines()
        .map(|line| Pair::new(line.trim()))
        .collect::<Vec<_>>();
    let count = pairs.iter().filter(|pair| pair.contains()).count();
    println!("Part 1: {}", count);

    let count = pairs.iter().filter(|pair| pair.overlap()).count();
    println!("Part 2: {}", count);
}

struct Pair {
    a: (u64, u64),
    b: (u64, u64),
}

impl Pair {
    fn new(input: &str) -> Self {
        let mut iter = input
            .split(',')
            .flat_map(|s| s.split("-").map(|s| s.parse::<u64>().unwrap()));
        Self {
            a: (iter.next().unwrap(), iter.next().unwrap()),
            b: (iter.next().unwrap(), iter.next().unwrap()),
        }
    }

    fn contains(&self) -> bool {
        contains(self.a, self.b) || contains(self.b, self.a)
    }
    fn overlap(&self) -> bool {
        overlap(self.a, self.b) || overlap(self.b, self.a)
    }
}

fn contains(a: (u64, u64), b: (u64, u64)) -> bool {
    a.0 <= b.0 && a.1 >= b.1
}
fn overlap(a: (u64, u64), b: (u64, u64)) -> bool {
    a.0 <= b.1 && a.1 >= b.0
}

#[cfg(test)]
#[test]
fn test_pair_contains() {
    let p = Pair::new("2-4,6-8");
    assert_eq!(p.contains(), false);
    assert_eq!(p.overlap(), false);
    let p = Pair::new("2-3,4-5");
    assert_eq!(p.contains(), false);
    assert_eq!(p.overlap(), false);
    let p = Pair::new("5-7,7-9");
    assert_eq!(p.contains(), false);
    assert_eq!(p.overlap(), true);
    let p = Pair::new("2-8,3-7");
    assert_eq!(p.contains(), true);
    assert_eq!(p.overlap(), true);
    let p = Pair::new("6-6,4-6");
    assert_eq!(p.contains(), true);
    assert_eq!(p.overlap(), true);
    let p = Pair::new("2-6,4-8");
    assert_eq!(p.contains(), false);
    assert_eq!(p.overlap(), true);
}
