use itertools::Itertools;

pub fn read_input() -> Vec<Line> {
    let input = include_str!("input.txt");
    
    parse_lines(input.lines().collect())
}

pub fn parse_lines(lines: Vec<&str>) -> Vec<Line> {
    lines.iter().map(|line| {
        let (p1,p2) = line.split(" -> ").
            map(|point| {
                let (v1,v2) = point.split(",").map(|vert| vert.parse().unwrap()).next_tuple().unwrap();
                Point{x:v1, y:v2}
            }).next_tuple().unwrap();
        Line{p1:p1, p2:p2}
    })
    .collect()
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Line {
    pub p1: Point,
    pub p2: Point,
}

impl Line {
    pub fn is_vertical(self: &Self) -> bool {
        self.p1.y == self.p2.y
    }

    pub fn is_horizontal(self: &Self) -> bool {
        self.p1.x == self.p2.x
    }

    pub fn slope(self: &Self) -> f64 {
        (self.p1.y -  self.p2.y) as f64 / (self.p1.x-self.p2.x) as f64
    }
    
    pub fn iter(self: &Self) -> LineIter {
        let x = if self.p2.x-self.p1.x >0 {
            1
        } else if self.p2.x-self.p1.x <0 {
            -1
        } else { 0 };
        let y = if self.p2.y-self.p1.y >0 {
            1
        } else if self.p2.y-self.p1.y <0 {
            -1
        } else { 0 };
        LineIter{
            cur: self.p1.clone(),
            stop: self.p2.clone(),
            dir: Point{x:x, y:y},
            finished: false,
        }
    }
}

pub struct LineIter {
    cur: Point,
    stop: Point,
    dir: Point,
    finished: bool,
}

impl Iterator for LineIter {
    type Item = Point;
    fn next(&mut self) -> Option<Self::Item> {
        if self.finished { return None };
        let ret = self.cur.clone();
        if self.cur == self.stop { self.finished = true};
        self.cur = Point{x:self.dir.x+self.cur.x, y: self.dir.y+self.cur.y};
        Some(ret)
    }
}