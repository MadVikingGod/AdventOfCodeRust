pub fn read_input() -> Map {
    let input = include_str!("input.txt");
    Map::new(input)
}

#[derive(Debug, Clone)]
pub struct Map {
    pub m: Vec<&'static str>
}

impl Map {
    pub fn new(input: &'static str) -> Map {
        Map{m: input.lines().collect()}
    }
    pub fn is_tree(&self, x: usize, y: usize) -> Result<bool, String> {
        if y>=self.m.len() {
            return Err("Index out of range".to_string())
        }
        Ok(self.m[y].chars().nth(x % self.m[y].len()).unwrap() == '#')
    }
    fn len(&self) -> usize {
        self.m.len()
    }
    pub fn slope(&self, dx: usize, dy: usize) -> Slope {
        Slope{m: self.clone(), dx: dx, dy: dy, x:0, y:0}
    }
}
#[derive(Debug, Clone)]
pub struct Slope {
    m: Map,
    dx: usize,
    dy: usize,
    x: usize,
    y: usize,
}
impl Iterator for Slope {
    type Item = bool;

    fn next(&mut self) -> Option<bool> {

        self.x += self.dx;
        self.y += self.dy;
        self.m.is_tree(self.x, self.y).ok()
    }
}
