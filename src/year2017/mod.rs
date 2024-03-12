use std::fs::read_to_string;

pub mod day1;

pub fn solve() {
    let content = read_to_string("inputs/Year2017/Day1.txt").unwrap();
    println!("{}", content);
}
