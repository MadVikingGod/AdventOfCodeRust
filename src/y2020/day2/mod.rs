use regex::Regex;

pub fn read_input() -> Vec<(Rule, String)> {
    let re = Regex::new(r"(\d+)-(\d+) (.): (.+)").unwrap();
    let input = include_str!("input.txt");
    input
        .lines()
        .map(|line| re.captures(line).unwrap())
        .map(|cap| (Rule::new(&cap[1], &cap[2], &cap[3]), String::from(&cap[4])))
        .collect()
}

#[derive(Debug)]
pub struct Rule {
    min: usize,
    max: usize,
    charactor: char,
}

impl Rule {
    pub fn new(min: &str, max: &str, charactor: &str) -> Rule {
        Rule {
            min: min.parse().unwrap(),
            max: max.parse().unwrap(),
            charactor: charactor.chars().next().unwrap(),
        }
    }
    pub fn check_password(&self, password: String) -> bool {
        let count = password.matches(self.charactor).count();
        count >= self.min && count <= self.max
    }
    pub fn check_toboggan_password(&self, password: String) -> bool {
        let c1 = password.chars().nth(self.min - 1).unwrap() == self.charactor;
        let c2 = password.chars().nth(self.max - 1).unwrap() == self.charactor;
        c1 ^ c2
    }
}
