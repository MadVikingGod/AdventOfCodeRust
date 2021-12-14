use advent_of_code::y2021::day14::*;
// Common tools
// use advent_of_code::util::Point;
// use advent_of_code::util::Field;
use std::collections::HashMap;
// use std::collections::HashSet;
// use std::collections::VecDeque;
use itertools::Itertools;


const TEST_INPUT : &str="NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

fn main() {
    println!("Hello, world!");

    let (inst, map) = parse_input(TEST_INPUT);
    println!("inst {}", inst);
    println!("map  {:?}", map);
    let mut formula = inst;
    for _ in 1..11 {
        formula = step(&formula, &map);
        // println!("After step {}: {}",i, formula);
    }
    let mut counts: HashMap<char, u64> = HashMap::new();
    formula.chars().for_each(|c| {
        *counts.entry(c).or_insert(0) += 1
    });
    println!("{:#?}", counts);

    let (inst, map) = read_input();
   
    let mut formula = inst;
    for _ in 1..11 {
        formula = step(&formula, &map);
        // println!("After step {}: {}",i, formula);
    }
    let mut counts: HashMap<char, u64> = HashMap::new();
    formula.chars().for_each(|c| {
        *counts.entry(c).or_insert(0) += 1
    });
    println!("{:#?}", counts);
    
   let (mut f, map) = read_adv_input();
   for _ in 0..40 {
       f = f.step(&map);
   }
   println!("{:#?}", f.to_count());
   println!("Remember add one to the last char of the formula 'O' :)");
}

fn step(input: &str, map: &HashMap<String,String>) -> String {
    let n1 = input.chars().take(input.len() - 1);
    let n2 = input.chars().skip(1);
    n1.zip(n2).map(|(c1,c2)| {
        let lookup: String = [c1,c2].iter().join("");
        map.get(&lookup).unwrap_or(&lookup).clone()
    }).collect::<String>() + &input[input.len()-1..]
}


// After step 1: NCNBCHB
// After step 1: NCNBCHB
// After step 2: NBCCNBBBCBHCB
// After step 3: NBBBCNCCNBBNBNBBCHBHHBCHB
// After step 4: NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB
// After step 2: NBCCNBBBCBHCB
// After step 3: NBBBCNCCNBBNBNBBCHBHHBCHB
// After step 4: NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB
// After step 5: NBBNBBNBBBNBBNBBCNCCNBBBCCNBCNCCNBBNBBNBBNBBNBBNBNBBNBBNBBNBBNBBCHBHHBCHBHHNHCNCHBCHBNBBCHBHHBCHB