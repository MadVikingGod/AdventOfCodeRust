pub fn read_input() -> Vec<u64> {
    let input = include_str!("input.txt");
    input.lines().map(|l| convert(l)).collect()
}

pub fn convert(s: &str) -> u64 {
    // F=0, B=1, L=0, R=1
    u64::from_str_radix(
        &s.replace("F", "0")
            .replace("B", "1")
            .replace("L", "0")
            .replace("R", "1"),
        2,
    )
    .unwrap_or(0)
}
