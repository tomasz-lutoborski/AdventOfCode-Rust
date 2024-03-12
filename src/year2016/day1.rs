use std::{
    char::ParseCharError, collections::HashSet, fs::read_to_string, num::ParseIntError,
    str::FromStr,
};

use itertools::{concat, Itertools};

#[derive(Debug, PartialEq, Eq)]
enum Turn {
    Left,
    Right,
}

impl FromStr for Turn {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "L" => Ok(Turn::Left),
            "R" => Ok(Turn::Right),
            _ => Err("wrong turn in instruction"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn get_new(direction: &Self, turn: Turn) -> Self {
        match turn {
            Turn::Left => match direction {
                Direction::Left => Direction::Down,
                Direction::Right => Direction::Up,
                Direction::Down => Direction::Right,
                Direction::Up => Direction::Left,
            },
            Turn::Right => match direction {
                Direction::Left => Direction::Up,
                Direction::Down => Direction::Left,
                Direction::Right => Direction::Down,
                Direction::Up => Direction::Right,
            },
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Instruction {
    turn: Turn,
    steps: i32,
}

impl FromStr for Instruction {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (turn_str, steps_str) = s.split_at(1);
        let turn = turn_str.parse::<Turn>()?;
        let steps = steps_str.parse::<i32>().map_err(|_| "invalid steps")?;

        Ok(Instruction { turn, steps })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Position {
    direction: Direction,
    x: i32,
    y: i32,
    visited: HashSet<(i32, i32)>,
}

impl Default for Position {
    fn default() -> Self {
        let mut visited = HashSet::new();
        visited.insert((0, 0));

        Position {
            direction: Direction::Up,
            x: 0,
            y: 0,
            visited,
        }
    }
}

impl Position {
    fn new(direction: Direction, x: i32, y: i32) -> Self {
        let mut visited = HashSet::new();
        visited.insert((0, 0));

        Position {
            direction,
            x,
            y,
            visited,
        }
    }

    fn get_visited(&self, direction: &Direction, steps: i32) -> Vec<(i32, i32)> {
        let (start_x, start_y) = (self.x, self.y);
        let mut points = Vec::new();

        match direction {
            Direction::Down => {
                for step in 1..=steps {
                    points.push((start_x, start_y - step));
                }
            }
            Direction::Up => {
                for step in 1..=steps {
                    points.push((start_x, start_y + step));
                }
            }
            Direction::Left => {
                for step in 1..=steps {
                    points.push((start_x - step, start_y));
                }
            }
            Direction::Right => {
                for step in 1..=steps {
                    points.push((start_x + step, start_y));
                }
            }
        }

        points
    }

    fn update(&mut self, instruction: Instruction) -> Vec<(i32, i32)> {
        let new_dir = Direction::get_new(&self.direction, instruction.turn);
        let visited = self.get_visited(&new_dir, instruction.steps);

        let (x, y) = match new_dir {
            Direction::Down => (self.x, self.y - instruction.steps),
            Direction::Up => (self.x, self.y + instruction.steps),
            Direction::Right => (self.x + instruction.steps, self.y),
            Direction::Left => (self.x - instruction.steps, self.y),
        };

        self.x = x;
        self.y = y;
        self.direction = new_dir;

        for (x, y) in visited.clone() {
            self.visited.insert((x, y));
        }

        visited
    }
}

fn get_instructions(s: &str) -> Vec<Instruction> {
    s.lines()
        .collect_vec()
        .get(0)
        .unwrap()
        .split(", ")
        .map(|inst| inst.parse().unwrap())
        .collect_vec()
}

fn get_distance(position: Position) -> i32 {
    position.x.abs() + position.y.abs()
}

// fn get_final_position(instructions: Vec<Instruction>) -> Position {
//     let mut curr_pos = Position::default();
//     for instruction in instructions {
//         curr_pos.update(instruction);
//     }

//     curr_pos
// }

fn get_final_position(instructions: Vec<Instruction>) -> Position {
    let mut curr_pos = Position::default();
    for instruction in instructions {
        let mut new_pos = curr_pos.clone();
        let visited = new_pos.update(instruction);
        if visited.iter().any(|coord| curr_pos.visited.contains(coord)) {
            break;
        };
        curr_pos = new_pos;
    }

    curr_pos
}

pub fn solve() {
    let content = read_to_string("inputs/Year2016/Day1.txt").unwrap();
    println!(
        "{:?}",
        get_distance(get_final_position(get_instructions(content.as_str())))
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_update_position() {
        let mut pos = Position::new(Direction::Up, 0, 0);
        let mut visited = HashSet::new();
        visited.insert((0, 0));
        visited.insert((-1, 0));
        visited.insert((-2, 0));
        pos.update(Instruction {
            turn: Turn::Left,
            steps: 2,
        });
        assert_eq!(
            pos,
            Position {
                direction: Direction::Left,
                x: -2,
                y: 0,
                visited: visited.clone()
            }
        );
        pos.update(Instruction {
            turn: Turn::Right,
            steps: 2,
        });
        pos.update(Instruction {
            turn: Turn::Right,
            steps: 1,
        });
        visited.insert((-2, 1));
        visited.insert((-2, 2));
        visited.insert((-1, 2));
        assert_eq!(
            pos,
            Position {
                direction: Direction::Right,
                x: -1,
                y: 2,
                visited: visited.clone()
            }
        );
    }

    #[test]
    fn instruction_from_string() {
        let inst1: Instruction = "L1".parse().unwrap();
        let inst2: Instruction = "R50".parse().unwrap();

        assert_eq!(
            inst1,
            Instruction {
                turn: Turn::Left,
                steps: 1
            }
        );

        assert_eq!(
            inst2,
            Instruction {
                turn: Turn::Right,
                steps: 50
            }
        );
    }

    #[test]
    fn test_get_instructions() {
        let insts = get_instructions("L5, R1, R4, L5, L4, R3\n");

        assert_eq!(
            insts,
            vec![
                Instruction {
                    turn: Turn::Left,
                    steps: 5
                },
                Instruction {
                    turn: Turn::Right,
                    steps: 1
                },
                Instruction {
                    turn: Turn::Right,
                    steps: 4
                },
                Instruction {
                    turn: Turn::Left,
                    steps: 5
                },
                Instruction {
                    turn: Turn::Left,
                    steps: 4
                },
                Instruction {
                    turn: Turn::Right,
                    steps: 3
                }
            ]
        )
    }
}
