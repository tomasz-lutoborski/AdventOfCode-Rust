use std::{collections::HashMap, fs::read_to_string, hash::Hash};

type Alphabet = HashMap<char, usize>;

fn construct_correct(s: &str) -> String {
    let mut decoded = String::new();
    for i in 0..8 {
        let mut counts = Alphabet::new();
        for line in s.lines() {
            counts
                .entry(line.chars().nth(i).unwrap())
                .and_modify(|e| *e += 1)
                .or_insert(1);
        }
        counts
            .iter()
            .min_by_key(|&(_, count)| count)
            .map(|(c, _)| decoded.push(*c));
    }
    decoded
}

pub fn solve() {
    let content = read_to_string("inputs/Year2016/Day6.txt").unwrap();
    println!("{}", construct_correct(&content));
}
