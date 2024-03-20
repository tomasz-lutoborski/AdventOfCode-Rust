use std::str::FromStr;

use anyhow::{anyhow, Result};
use regex::Regex;

type Grid = Vec<Vec<bool>>;

type Grid2 = Vec<Vec<i32>>;

#[derive(Debug, PartialEq, Eq)]
enum Action {
    TurnOn,
    TurnOff,
    Toggle,
}

#[derive(Debug)]
struct Instruction {
    action: Action,
    start: (usize, usize),
    end: (usize, usize),
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(
            r"^(?P<action>turn on|turn off|toggle) (?P<start1>\d+),(?P<start2>\d+) through (?P<end1>\d+),(?P<end2>\d+)$",
        )?;
        let caps = re.captures(s).ok_or(anyhow!("Invalid instruction"))?;
        let action = match caps.name("action").unwrap().as_str() {
            "turn on" => Action::TurnOn,
            "turn off" => Action::TurnOff,
            "toggle" => Action::Toggle,
            _ => return Err(anyhow!("Invalid action")),
        };
        let start = (
            caps.name("start1").unwrap().as_str().parse()?,
            caps.name("start2").unwrap().as_str().parse()?,
        );
        let end = (
            caps.name("end1").unwrap().as_str().parse()?,
            caps.name("end2").unwrap().as_str().parse()?,
        );
        Ok(Instruction { action, start, end })
    }
}

fn apply_instruction(grid: &mut Grid, instruction: &Instruction) {
    for i in instruction.start.0..=instruction.end.0 {
        for j in instruction.start.1..=instruction.end.1 {
            match instruction.action {
                Action::TurnOn => grid[i][j] = true,
                Action::TurnOff => grid[i][j] = false,
                Action::Toggle => grid[i][j] = !grid[i][j],
            }
        }
    }
}

fn apply_instruction2(grid: &mut Grid2, instruction: &Instruction) {
    for i in instruction.start.0..=instruction.end.0 {
        for j in instruction.start.1..=instruction.end.1 {
            match instruction.action {
                Action::TurnOn => grid[i][j] += 1,
                Action::TurnOff => {
                    if grid[i][j] > 0 {
                        grid[i][j] -= 1
                    }
                }
                Action::Toggle => grid[i][j] += 2,
            }
        }
    }
}

fn apply_instructions(input: &str) -> usize {
    let mut grid = vec![vec![false; 1000]; 1000];
    for line in input.lines() {
        let instruction = line.parse::<Instruction>().unwrap();
        apply_instruction(&mut grid, &instruction);
    }
    grid.iter().flatten().filter(|&&x| x).count()
}

fn apply_instructions2(input: &str) -> i32 {
    let mut grid = vec![vec![0; 1000]; 1000];
    for line in input.lines() {
        let instruction = line.parse::<Instruction>().unwrap();
        apply_instruction2(&mut grid, &instruction);
    }
    grid.iter().flatten().sum()
}

pub fn solve() {
    let content = std::fs::read_to_string("inputs/Year2015/Day6.txt").unwrap();
    println!("{}", apply_instructions2(&content));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_instruction_from_str() {
        let input = "turn on 0,0 through 999,999";
        let instruction = input.parse::<Instruction>().unwrap();
        assert_eq!(instruction.action, Action::TurnOn);
        assert_eq!(instruction.start, (0, 0));
        assert_eq!(instruction.end, (999, 999));
    }

    #[test]
    fn test_apply_instructions2() {
        let input = "turn on 0,0 through 10,10\n\
                     turn on 0,0 through 10,10";
        assert_eq!(apply_instructions2(input), 242);
    }
}
