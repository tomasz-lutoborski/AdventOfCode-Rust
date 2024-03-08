use std::{error, fs::read_to_string};

fn parse_char(acc: i64, c: char) -> i64 {
    match c {
        '(' => acc + 1,
        ')' => acc - 1,
        _ => panic!("wrong input char"),
    }
}

fn find_basement(input: String) -> usize {
    let mut current_level = 0;
    for c in input.char_indices() {
        current_level = parse_char(current_level, c.1);
        if current_level < 0 {
            return c.0 + 1;
        };
    }

    0
}

fn parse_inst(input: String) -> i64 {
    input.chars().fold(0, parse_char)
}

pub fn solve() {
    let file_content = read_to_string("inputs/Year2015/Day1.txt").unwrap();
    println!("{}", find_basement(file_content));
}
