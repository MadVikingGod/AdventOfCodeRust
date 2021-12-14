pub fn read_input() -> Vec<Line> {
    let input = include_str!("input.txt");
    input
        .lines()
        .map(|line| Line {
            c: line.chars().collect(),
        })
        .collect()
}

#[derive(Debug, Clone)]
pub struct Line {
    pub c: Vec<char>,
}

impl Line {
    pub fn invalid(&self) -> Option<char> {
        let mut stack: Vec<char> = vec![];
        let pos = self.c.iter();
        for c in pos {
            if "[{(<".contains(*c) {
                stack.push(*c);
            } else {
                let comp = match stack.pop()? {
                    '[' => ']',
                    '{' => '}',
                    '(' => ')',
                    '<' => '>',
                    _ => return Some('X'),
                };
                if comp != *c {
                    return Some(*c);
                };
            };
        }
        None
    }

    pub fn incomplete(&self) -> Option<String> {
        let mut stack: Vec<char> = vec![];
        let pos = self.c.iter();
        for c in pos {
            if "[{(<".contains(*c) {
                stack.push(*c);
            } else {
                let comp = match stack.pop()? {
                    '[' => ']',
                    '{' => '}',
                    '(' => ')',
                    '<' => '>',
                    _ => return None,
                };
                if comp != *c {
                    return None;
                };
            };
        }
        Some(stack.iter().collect())
    }
}
