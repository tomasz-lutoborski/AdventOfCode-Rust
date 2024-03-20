use itertools::Itertools;
use std::fs::read_to_string;

fn part1(s: &str) -> (usize, usize) {
    let mut chars_data = 0;
    let mut chars_code = 0;

    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        chars_code += 1;

        match c {
            '\\' => match chars.peek() {
                Some('x') => {
                    chars_code += 3;
                    chars.next();
                    chars.next();
                    chars.next();
                    chars_data += 1;
                }
                Some(_) => {
                    chars.next();
                    chars_data += 1;
                    chars_code += 1;
                }
                None => {}
            },
            '"' => {}
            _ => {
                chars_data += 1;
            }
        }
    }

    (chars_data, chars_code)
}

fn part2(s: &str) -> (usize, usize) {
    let mut chars_data = 2;
    let mut chars_code = 0;

    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        chars_code += 1;

        match c {
            '\\' => {
                chars_data += 2;
            }
            '"' => {
                chars_data += 2;
            }
            _ => {
                chars_data += 1;
            }
        }
    }

    (chars_data, chars_code)
}

pub fn solve() {
    let content = read_to_string("inputs/Year2015/Day8.txt").unwrap();
    let result = content
        .lines()
        .map(part2)
        .fold((0, 0), |acc, (data, code)| (acc.0 + data, acc.1 + code));
    println!("Part 1: {}", result.0 - result.1);
}

#[cfg(test)]
mod test {
    use super::*;
    static INPUT: &str = r#"""
"abc"
"aaa\"aaa"
"\x27""#;

    #[test]
    fn test_input() {
        for line in INPUT.lines() {
            println!("{}", line.len());
        }
        assert_eq!(INPUT.lines().take(1).collect_vec().len(), 1);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(r#""""#), (0, 2));
        assert_eq!(part1(r#""abc""#), (3, 5));
        assert_eq!(part1(r#""aaa\"aaa""#), (7, 10));
        assert_eq!(part1(r#""\x27""#), (1, 6));
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(r#""""#), (6, 2));
        assert_eq!(part2(r#""abc""#), (9, 5));
        assert_eq!(part2(r#""aaa\"aaa""#), (16, 10));
        assert_eq!(part2(r#""\x27""#), (11, 6));
    }
}
