// Common tools
// use crate::util::Field;
// use crate::util::Point;
// use std::collections::HashMap;
// use std::collections::HashSet;
// use std::collections::VecDeque;
use itertools::Itertools;
use std::{convert::TryInto, fmt::Display, ops::Add};

pub fn read_input() -> Vec<Pair> {
    let input = include_str!("input.txt");

    input.lines().map(|l| {
        l.into()
    }).collect()
}

pub fn parse_input(input: &str) -> Vec<Pair> {
    input.lines().map(|l| l.into()).collect()
}

#[derive(Debug, Clone, Copy)]
pub enum Element {
    Start,
    Value(i32),
    End
}

#[derive(Debug, Clone)]
pub struct Pair{
    pub elements: Vec<Element>
}

pub fn add_all(input: Vec<Pair>) -> Pair {
    input.into_iter().reduce(|acc, pair| {
        let p = acc + pair;
        p.reduce()
    }).unwrap()
}

pub fn add_pairs(input: Vec<Pair>) -> i32 {
    let sums = input.iter().permutations(2)
        .map(|v| (v[0].to_owned(), v[1].to_owned()))
        .map(|(l,r)| l + r)
        .map(|s| s.reduce().magnitude());
        
    
    sums.max().unwrap()
}

impl Pair {
    pub fn explode(&self) -> Option<Self> {
        let mut depth = 0;
        let len = self.elements.len();
        let eles = self.elements.iter()
            .take(len-4)
            .enumerate();
        for (i, element) in eles {
            match (depth, element, self.elements[i+1], self.elements[i+2], self.elements[i+3]) {
                (4.., Element::Start, Element::Value(l), Element::Value(r), Element::End) => {
                    let left = self.add_left(i,l);
                    let right = self.add_right(i+4,r);
                    let eles = [left, vec![Element::Value(0)], right].concat();
                    return Some(Pair { elements: eles })
                },
                (_, Element::Start,_,_,_) => depth+=1,
                (_, Element::End,_,_,_) => depth-=1,
                _ => ()
            };
        };
        None
    }

    fn add_left(&self, i:usize, n: i32) -> Vec<Element>{
        let mut elements: Vec<Element> = self.elements[..i].to_owned();

        for (i, val) in self.elements[..i].iter().enumerate().rev() {
            match val {
                Element::Value(val) => {
                    elements[i] = Element::Value(val + n);
                    break
                }
                _ => (),
            };
        }
        elements
        
    } 
    fn add_right(&self, i:usize, n:i32) -> Vec<Element>{
        let mut elements: Vec<Element>  = self.elements[i..].to_owned();
        
        for (i, val) in self.elements[i..].iter().enumerate() {
            match val {
                Element::Value(val) => {
                    elements[i] = Element::Value(val +n);
                    break
                },
                _ => (),
            }
        };

        elements
    }   

    pub fn split(&self) -> Option<Self> {
        for (i, ele) in self.elements.iter().enumerate() {
            match ele {
                Element::Value(v) if v >= &10 => {
                    let left = self.elements[..i].to_vec();
                    let right = self.elements[i+1..].to_vec();
                    let new = vec![Element::Start, Element::Value(v/2), Element::Value(v.div_ceil(2)), Element::End];
                    let eles = [left, new, right].concat();
                    return Some(Pair{elements: eles})
                } 
                _=>()
            }
        };
        None
    }

    pub fn reduce(&self) -> Self {
        let exp = self.explode();
        if exp.is_some() {
            return exp.unwrap().reduce()
        }
        let sp = self.split();
        if sp.is_some() {
            return sp.unwrap().reduce()
        }
        self.to_owned()
    }

    pub fn magnitude(&self) -> i32 {
        let mut queue: Vec<i32> = Vec::new();
        self.elements.clone().into_iter().for_each(|ele| {
            match ele {
                Element::Value(v) => queue.push(v),
                Element::End => {
                    let r = queue.pop().unwrap();
                    let l = queue.pop().unwrap();
                    queue.push(3*l+2*r)
                }
                _ =>()
            };
        });
        queue.pop().unwrap()
    }
}

impl From<&str> for Pair {
    //This assumes that s is a line containing a Snailfish pair
    fn from(s: &str) -> Self {
        Self{
            elements: s.chars().filter_map(|c| {
                match c {
                    '[' => Some(Element::Start),
                    ']' => Some(Element::End),
                    c if c.is_digit(10) => Some(Element::Value(c.to_digit( 10).unwrap().try_into().unwrap())),
                    _ => None
                }
            }).collect_vec()
        }
    }
}
impl Display for Pair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut prev: Element = Element::Start;
        for ele in &self.elements {
            match (ele, prev) {
                (Element::Start, Element::Start) => write!(f, "[")?,
                (Element::Start, Element::Value(_)) => write!(f, ",[")?,
                (Element::Start, Element::End) => write!(f, ",[")?,
                (Element::Value(v), Element::Start) => write!(f, "{}", v)?,
                (Element::Value(v), Element::Value(_)) => write!(f, ",{}",v)?,
                (Element::Value(v), Element::End) => write!(f, ",{}",v)?,
                (Element::End, Element::Start) => write!(f, "]")?,
                (Element::End, Element::Value(_)) => write!(f, "]")?,
                (Element::End, Element::End) => write!(f, "]")?,
            };
            prev = *ele
        };
        Ok(())
    }
}

impl Add for Pair {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Pair { elements: [vec![Element::Start], self.elements, rhs.elements, vec![Element::End]].concat() }
    }
}