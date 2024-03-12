use std::{fs::read_to_string, str::FromStr, vec};

use itertools::{Itertools, Position};

#[derive(Debug, PartialEq, Eq)]
struct Keypad {
    pos: (usize, usize),
    keypad: Vec<Vec<i8>>,
}

impl Keypad {
    fn new() -> Self {
        Keypad {
            pos: (1, 1),
            keypad: vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]],
        }
    }

    fn update_pos(&mut self, direction: Direction) {
        let mut new_x = self.pos.0;
        let mut new_y = self.pos.1;
        match direction {
            Direction::Up => match self.pos {
                (_, 0) => (),
                _ => new_y = new_y - 1,
            },
            Direction::Down => match self.pos {
                (_, 2) => (),
                _ => new_y = new_y + 1,
            },
            Direction::Left => match self.pos {
                (0, _) => (),
                _ => new_x = new_x - 1,
            },
            Direction::Right => match self.pos {
                (2, _) => (),
                _ => new_x = new_x + 1,
            },
        }

        self.pos = (new_x, new_y);
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl FromStr for Direction {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "R" => Ok(Self::Right),
            "L" => Ok(Self::Left),
            "D" => Ok(Self::Down),
            "U" => Ok(Self::Up),
            _ => Err("bad character"),
        }
    }
}

fn parse_instructions(input: &str) -> Vec<Direction> {
    input
        .chars()
        .map(|c| c.to_string().parse().unwrap())
        .collect_vec()
}

fn get_code(input: &str) -> String {
    let instructions_lines = input.lines().map(|l| parse_instructions(l));

    let mut result = String::new();

    for line in instructions_lines {
        let mut keypad = Keypad::new();
        for instruction in line {
            keypad.update_pos(instruction);
        }

        let final_digit = keypad.keypad[keypad.pos.1][keypad.pos.0];
        result.push(char::from_digit(final_digit as u32, 10).unwrap());
    }

    result
}

pub fn solve() {
    let content = read_to_string("inputs/Year2016/Day2.txt").unwrap();
    println!("{:?}", get_code(content.as_str()));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_instructions() {
        assert_eq!(
            parse_instructions("RDRRD"),
            vec![
                Direction::Right,
                Direction::Down,
                Direction::Right,
                Direction::Right,
                Direction::Down
            ]
        )
    }

    #[test]
    fn test_update_pos() {
        let mut keypad = Keypad::new();
        keypad.update_pos(Direction::Down);
        assert_eq!(keypad.pos, (1, 2));

        keypad.update_pos(Direction::Right);
        keypad.update_pos(Direction::Right);
        assert_eq!(keypad.pos, (2, 2));

        keypad.update_pos(Direction::Up);
        keypad.update_pos(Direction::Up);
        keypad.update_pos(Direction::Up);
        assert_eq!(keypad.pos, (2, 0));

        keypad.update_pos(Direction::Left);
        keypad.update_pos(Direction::Left);
        keypad.update_pos(Direction::Left);
        assert_eq!(keypad.pos, (0, 0));
    }

    #[test]
    fn test_get_code() {
        assert_eq!(get_code("DRRUUULLL\n"), "1".to_string());
        assert_eq!(get_code("DDDLLUUUURD\n"), "5".to_string());
    }
}
