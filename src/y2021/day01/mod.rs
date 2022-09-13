pub fn read_input() -> Vec<i64> {
    let bytes = include_str!("input.txt");
    bytes.lines().map(|x| x.parse().unwrap()).collect()
}

pub fn count_increase(l: &[i64]) -> usize {
    l.windows(2).filter(|w| w[0] < w[1]).count()
    // l[0..l.len()-1].iter().zip(l[1..].iter()).filter(|(x,y)| x<y).count()
}

pub fn count_window_increase(l: &[i64], n: usize) -> usize {
    if n >= l.len() {
        return 0;
    }
    l.windows(n)
        .map(|w| w.iter().sum::<i64>())
        .collect::<Vec<i64>>()
        .windows(2)
        .filter(|w| w[0] < w[1])
        .count()
}
