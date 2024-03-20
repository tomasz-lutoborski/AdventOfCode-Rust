use anyhow::{anyhow, Result};
use std::{fs::read_to_string, str::FromStr};

type Screen = [[bool; 50]; 6];

#[derive(Debug, PartialEq)]
enum Instruction {
    Rect(usize, usize),
    RotateRow(usize, usize),
    RotateColumn(usize, usize),
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let parts: Vec<&str> = s.split_whitespace().collect();
        match parts[0] {
            "rect" => {
                let dimensions: Vec<usize> =
                    parts[1].split('x').map(|s| s.parse().unwrap()).collect();
                Ok(Instruction::Rect(dimensions[0], dimensions[1]))
            }
            "rotate" => {
                let index: usize = parts[2][2..].parse().unwrap();
                let amount: usize = parts[4].parse().unwrap();
                match parts[1] {
                    "row" => Ok(Instruction::RotateRow(index, amount)),
                    "column" => Ok(Instruction::RotateColumn(index, amount)),
                    _ => Err(anyhow!("Invalid rotation")),
                }
            }
            _ => Err(anyhow!("Invalid instruction")),
        }
    }
}

fn apply_instructions(s: &str) -> Screen {
    let mut screen: Screen = [[false; 50]; 6];
    for line in s.lines() {
        let instruction: Instruction = line.parse().unwrap();
        match instruction {
            Instruction::Rect(x, y) => {
                for i in 0..y {
                    for j in 0..x {
                        screen[i][j] = true;
                    }
                }
            }
            Instruction::RotateRow(y, amount) => {
                let mut new_row = [false; 50];
                for i in 0..50 {
                    new_row[(i + amount) % 50] = screen[y][i];
                }
                screen[y] = new_row;
            }
            Instruction::RotateColumn(x, amount) => {
                let mut new_column = [false; 6];
                for i in 0..6 {
                    new_column[(i + amount) % 6] = screen[i][x];
                }
                for i in 0..6 {
                    screen[i][x] = new_column[i];
                }
            }
        }
    }
    screen
}

fn draw_screen(screen: &Screen) {
    for row in screen.iter() {
        for &cell in row.iter() {
            print!("{}", if cell { '#' } else { '.' });
        }
        println!();
    }
}

pub fn solve() {
    let content = read_to_string("inputs/Year2016/Day8.txt").unwrap();

    let screen = apply_instructions(&content);

    draw_screen(&screen);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_instruction() {
        assert_eq!(
            "rect 3x2".parse::<Instruction>().unwrap(),
            Instruction::Rect(3, 2)
        );
        assert_eq!(
            "rotate column x=1 by 1".parse::<Instruction>().unwrap(),
            Instruction::RotateColumn(1, 1)
        );
        assert_eq!(
            "rotate row y=0 by 4".parse::<Instruction>().unwrap(),
            Instruction::RotateRow(0, 4)
        );
    }
}
