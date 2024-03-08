use std::{collections::HashMap, fs::read_to_string};

#[derive(PartialEq, Eq, Debug, Hash, Copy, Clone)]
struct Cell {
    x: i32,
    y: i32,
}

enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug)]
struct Visited {
    cells: HashMap<Cell, i32>,
    current_cell: Cell,
}

impl Visited {
    fn new() -> Self {
        let mut cells = HashMap::new();
        cells.insert(Cell { x: 0, y: 0 }, 1);

        Visited {
            cells,
            current_cell: Cell { x: 0, y: 0 },
        }
    }

    fn visit_cell(&mut self, dir: Direction) {
        self.current_cell = match dir {
            Direction::Left => Cell {
                x: self.current_cell.x - 1,
                y: self.current_cell.y,
            },
            Direction::Right => Cell {
                x: self.current_cell.x + 1,
                y: self.current_cell.y,
            },
            Direction::Up => Cell {
                x: self.current_cell.x,
                y: self.current_cell.y + 1,
            },
            Direction::Down => Cell {
                x: self.current_cell.x,
                y: self.current_cell.y - 1,
            },
        };

        self.cells
            .entry(self.current_cell)
            .and_modify(|e| *e += 1)
            .or_insert(1);
    }

    fn parse_move(&mut self, inst: char) {
        match inst {
            '<' => self.visit_cell(Direction::Left),
            '>' => self.visit_cell(Direction::Right),
            '^' => self.visit_cell(Direction::Up),
            'v' => self.visit_cell(Direction::Down),
            _ => panic!("wrong instruction"),
        }
    }
}

pub fn solve() {
    let file_content = read_to_string("inputs/Year2015/Day3.txt").unwrap();
    let mut visited_santa = Visited::new();
    let mut visited_robot = Visited::new();
    let chars_santa = file_content.chars().step_by(2);
    let chars_robot = file_content.chars().skip(1).step_by(2);

    for inst in chars_santa {
        visited_santa.parse_move(inst);
    }

    for inst in chars_robot {
        visited_robot.parse_move(inst);
    }

    let visited = for (key, value) in visited_robot.cells.iter() {
        visited_santa
            .cells
            .entry(*key)
            .and_modify(|v| *v += value)
            .or_insert(*value);
    };

    let n_visited_more_then_once = visited_santa.cells.into_values().count();
    println!("{:?}", n_visited_more_then_once);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_visit_cell() {
        let mut visited = Visited::new();
        visited.visit_cell(Direction::Right);
        visited.visit_cell(Direction::Down);
        visited.visit_cell(Direction::Down);
        visited.visit_cell(Direction::Up);

        assert_eq!(visited.cells.get(&Cell { x: 1, y: 0 }), Some(&1));
        assert_eq!(visited.cells.get(&Cell { x: 1, y: -1 }), Some(&2));
        assert_eq!(visited.cells.get(&Cell { x: 1, y: -11 }), None);
    }

    #[test]
    fn test_n_of_visited_more_then_once() {
        let mut visited1 = Visited::new();
        visited1.visit_cell(Direction::Right);
        visited1.visit_cell(Direction::Down);
        visited1.visit_cell(Direction::Down);
        visited1.visit_cell(Direction::Up);
        visited1.visit_cell(Direction::Up);

        assert_eq!(visited1.cells.into_values().filter(|&n| n > 1).count(), 2);

        let mut visited2 = Visited::new();
        visited2.visit_cell(Direction::Right);
        visited2.visit_cell(Direction::Down);
        visited2.visit_cell(Direction::Left);
        visited2.visit_cell(Direction::Up);

        assert_eq!(visited2.cells.into_values().filter(|&n| n > 1).count(), 1);

        let mut visited3 = Visited::new();
        visited3.visit_cell(Direction::Right);
        visited3.visit_cell(Direction::Left);
        visited3.visit_cell(Direction::Right);
        visited3.visit_cell(Direction::Left);
        visited3.visit_cell(Direction::Right);
        visited3.visit_cell(Direction::Left);
        visited3.visit_cell(Direction::Right);
        visited3.visit_cell(Direction::Left);
        visited3.visit_cell(Direction::Right);
        visited3.visit_cell(Direction::Left);

        assert_eq!(visited3.cells.into_values().filter(|&n| n > 1).count(), 2);
    }
}
